use eframe::egui::{self, Color32};
use egui_plot::{Line, MarkerShape, Plot, PlotPoint, Points};
use std::{
    path::{Path, PathBuf},
    sync::mpsc::{self, Receiver, TryRecvError},
};
use tempo2_rust::bats::Number;

const ACTIVE: Color32 = Color32::from_rgb(72, 160, 225);
const EXCLUDED: Color32 = Color32::from_rgb(231, 153, 69);

enum Action {
    Load,
    Exclude(usize, bool),
    Refit,
    Save(PathBuf),
}

struct WorkerReply {
    session: Option<tempo2_gui::Session>,
    message: Result<Option<String>, String>,
}

struct Viewer {
    par: String,
    tim: String,
    runtime: String,
    pending: Option<Receiver<WorkerReply>>,
    session: Option<tempo2_gui::Session>,
    rows: Result<Vec<tempo2_gui::PlotToa>, String>,
    selected: Option<usize>,
    selected_input: Option<usize>,
    error: Option<String>,
    message: Option<String>,
    save_directory: String,
    reset_plot: bool,
}

impl Viewer {
    fn new() -> Self {
        Self {
            par: String::new(),
            tim: String::new(),
            runtime: std::env::var("TEMPO2").unwrap_or_default(),
            pending: None,
            session: None,
            rows: Ok(vec![]),
            selected: None,
            selected_input: None,
            error: None,
            message: None,
            save_directory: String::new(),
            reset_plot: false,
        }
    }

    fn start(&mut self, action: Action, ctx: &egui::Context) {
        let (par, tim, runtime) = (self.par.clone(), self.tim.clone(), self.runtime.clone());
        let (sender, receiver) = mpsc::channel();
        let ctx = ctx.clone();
        self.pending = Some(receiver);
        self.selected_input = if matches!(action, Action::Load) {
            None
        } else {
            self.session
                .as_ref()
                .and_then(|s| {
                    self.selected
                        .and_then(|i| s.timing().pulsars[0].observations.get(i))
                })
                .and_then(|row| row.input_index)
        };
        let session = self.session.take();
        self.selected = None;
        self.error = None;
        self.message = None;
        std::thread::spawn(move || {
            let reply = match action {
                Action::Load => {
                    match tempo2_gui::load(Path::new(&par), Path::new(&tim), Path::new(&runtime)) {
                        Ok(session) => WorkerReply {
                            session: Some(session),
                            message: Ok(None),
                        },
                        Err(e) => WorkerReply {
                            session: None,
                            message: Err(e.to_string()),
                        },
                    }
                }
                action => {
                    let mut session = session.expect("operation enabled for a loaded session");
                    let message = match action {
                        Action::Exclude(index, excluded) => session
                            .set_excluded(index, excluded)
                            .map(|()| Some("Selection updated. Refit before saving.".into())),
                        Action::Refit => session.refit().map_err(|e| e.to_string()).map(|()| {
                            let count = session.fit_result().unwrap().cycles.len();
                            Some(if count == 0 {
                                "No fit cycles returned (NITS is zero).".into()
                            } else {
                                format!("Fit completed: {count} cycle(s).")
                            })
                        }),
                        Action::Save(path) => session
                            .save(&path)
                            .map(|()| Some(format!("Saved session to {}", path.display()))),
                        Action::Load => unreachable!(),
                    };
                    WorkerReply {
                        session: Some(session),
                        message,
                    }
                }
            };
            let _ = sender.send(reply); // The user may have closed the window.
            ctx.request_repaint();
        });
    }

    fn receive(&mut self) {
        let received = match self.pending.as_ref().map(Receiver::try_recv) {
            Some(Ok(reply)) => reply,
            Some(Err(TryRecvError::Disconnected)) => WorkerReply {
                session: None,
                message: Err("Timing worker stopped before returning a result".into()),
            },
            _ => return,
        };
        self.pending = None;
        if let Some(session) = received.session {
            let pulsar = &session.timing().pulsars[0];
            self.rows = tempo2_gui::plot_toas(pulsar);
            self.selected = self
                .selected_input
                .and_then(|index| {
                    pulsar
                        .observations
                        .iter()
                        .position(|row| row.input_index == Some(index))
                })
                .or_else(|| (!pulsar.observations.is_empty()).then_some(0));
            self.session = Some(session);
            self.reset_plot = true;
        }
        match received.message {
            Ok(message) => self.message = message,
            Err(error) => self.error = Some(error),
        }
    }
}

impl eframe::App for Viewer {
    fn ui(&mut self, ui: &mut egui::Ui, _: &mut eframe::Frame) {
        self.receive();
        let mut action = None;
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("TEMPO2-Rust · Timing");
            ui.label("Loading evaluates with mean removal. Refit uses loaded file fit flags/NITS and enables error rescaling.");
            ui.add_enabled_ui(self.pending.is_none(), |ui| {
                egui::Grid::new("inputs").num_columns(2).show(ui, |ui| {
                    for (label, value) in [("PAR file", &mut self.par), ("TIM file", &mut self.tim), ("TEMPO2 resources", &mut self.runtime)] {
                        let label = ui.label(label);
                        ui.add(egui::TextEdit::singleline(value).desired_width(f32::INFINITY)).labelled_by(label.id);
                        ui.end_row();
                    }
                });
                if ui.button("Load and evaluate").clicked() { action = Some(Action::Load); }
            });
            if self.pending.is_some() { ui.horizontal(|ui| { ui.spinner(); ui.label("Working…"); }); }
            if let Some(error) = &self.error { ui.colored_label(Color32::LIGHT_RED, error); }
            if let Some(message) = &self.message { ui.label(message); }
            let Some(session) = &self.session else { return; };
            let result = session.timing();
            let pulsar = &result.pulsars[0];
            ui.horizontal(|ui| {
                if ui.button("Refit").clicked() { action = Some(Action::Refit); }
                let label = ui.label("New save folder");
                ui.text_edit_singleline(&mut self.save_directory).labelled_by(label.id);
                if ui.add_enabled(session.can_save() && !self.save_directory.is_empty(), egui::Button::new("Save session")).clicked() {
                    action = Some(Action::Save(PathBuf::from(&self.save_directory)));
                }
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.strong(pulsar.name.text.as_deref().unwrap_or("Non-UTF8 pulsar name"));
                ui.label(format!("{} TOAs", pulsar.observations.len()));
                ui.colored_label(ACTIVE, "Active (circles)");
                ui.colored_label(EXCLUDED, "Excluded (crosses)");
                if ui.button("Reset view").clicked() { self.reset_plot = true; }
            });
            ui.label("Scroll or pinch to zoom · Drag to pan · Click a TOA to inspect · Double-click to reset");
            match &self.rows {
                Ok(rows) => {
                    let mut plot = Plot::new("residuals").id(egui::Id::unique("residuals")).x_axis_label("Site arrival time (MJD)")
                        .y_axis_label("Residual (µs)").height((ui.available_height() * 0.55).max(220.));
                    if self.reset_plot { plot = plot.reset(); self.reset_plot = false; }
                    let response = plot.show(ui, |plot| {
                        for row in rows {
                            let color = if row.excluded { EXCLUDED } else { ACTIVE };
                            plot.line(Line::new("", vec![[row.sat_mjd, row.residual_us - row.uncertainty_us],
                                [row.sat_mjd, row.residual_us + row.uncertainty_us]])
                                .id(("uncertainty", row.observation)).allow_hover(false).color(color).width(1.));
                        }
                        for (excluded, color, shape) in [(false, ACTIVE, MarkerShape::Circle), (true, EXCLUDED, MarkerShape::Cross)] {
                            let points: Vec<_> = rows.iter().filter(|r| r.excluded == excluded)
                                .map(|r| [r.sat_mjd, r.residual_us]).collect();
                            plot.points(Points::new(if excluded { "Excluded" } else { "Active" }, points).color(color).shape(shape).radius(4.));
                        }
                        if let Some(selected) = self.selected.and_then(|i| rows.get(i)) {
                            plot.points(Points::new("Selected TOA", vec![[selected.sat_mjd, selected.residual_us]])
                                .color(Color32::YELLOW).radius(7.).filled(false));
                        }
                    });
                    if response.response.clicked() && let Some(pointer) = response.response.interact_pointer_pos() {
                        self.selected = rows.iter().filter_map(|row| {
                            let position = response.transform.position_from_point(&PlotPoint::new(row.sat_mjd, row.residual_us));
                            let distance = position.distance_sq(pointer);
                            (distance <= 144.).then_some((row.observation, distance))
                        }).min_by(|a, b| a.1.total_cmp(&b.1)).map(|(index, _)| index);
                    }
                }
                Err(error) => { ui.colored_label(Color32::LIGHT_RED, format!("Plot unavailable: {error}. Exact TOA values remain available below.")); }
            }
            ui.separator();
            ui.horizontal(|ui| {
                let label = ui.label("Inspect TOA");
                egui::ComboBox::from_id_salt("toa").selected_text(self.selected.map(|i| (i + 1).to_string()).unwrap_or_else(|| "Select".into()))
                    .show_ui(ui, |ui| {
                        for (index, row) in pulsar.observations.iter().enumerate() {
                            let name = row.barycentric.filename.text.as_deref().unwrap_or("Non-UTF8 filename");
                            ui.selectable_value(&mut self.selected, Some(index), format!("{} · {name}", index + 1));
                        }
                    }).response.labelled_by(label.id);
                if let Some(row) = self.selected.and_then(|i| pulsar.observations.get(i)) {
                    let excluded = row.barycentric.deleted != 0;
                    if ui.add_enabled(row.input_index.is_some(), egui::Button::new(if excluded { "Include TOA" } else { "Exclude TOA" })).clicked() {
                        action = Some(Action::Exclude(row.input_index.unwrap(), !excluded));
                    }
                }
            });
            egui::ScrollArea::vertical().id_salt("details").show(ui, |ui| {
                if let Some(row) = self.selected.and_then(|i| pulsar.observations.get(i)) {
                    let b = &row.barycentric;
                    let identity = row.input_index.map(|i| (i + 1).to_string()).unwrap_or_else(|| "Generated TOA".into());
                    ui.label(format!("Input TOA: {identity} · Deleted flag: {}", b.deleted));
                    for (label, number) in [("SAT (MJD)", &b.sat_mjd), ("BAT (MJD)", &b.bat_mjd),
                        ("Residual (s)", &row.residual_s), ("Phase (turn)", &row.phase_turn)] {
                        ui.horizontal(|ui| { ui.label(label); ui.monospace(number_text(number)); });
                    }
                    if let Some(sigma) = &b.uncertainty.value { ui.label(format!("TOA uncertainty: {} {}", number_text(sigma), b.uncertainty.unit)); }
                    ui.collapsing("Complete TOA record", |ui| { ui.monospace(serde_json::to_string_pretty(row).expect("serializable timing record")); });
                }
                ui.collapsing(format!("Engine diagnostics ({})", result.diagnostics.len()), |ui| {
                    ui.monospace(serde_json::to_string_pretty(&result.diagnostics).expect("serializable diagnostics"));
                });
                if let Some(fit) = session.fit_result() {
                    ui.collapsing("Fit result", |ui| { ui.monospace(serde_json::to_string_pretty(fit).expect("serializable fit result")); });
                }
                ui.collapsing("Loaded inputs and resources", |ui| {
                    for input in &result.inputs { ui.label(format!("{} · {}", input.role, input.path.text.as_deref().unwrap_or("Non-UTF8 path"))); }
                    ui.label(format!("{} consumed resource records", result.resources.len()));
                });
            });
        });
        if let Some(action) = action {
            self.start(action, ui.ctx());
        }
    }
}

fn number_text(value: &Number) -> &str {
    match value {
        Number::Finite(value) => value,
        Number::Nonfinite { class } => class,
    }
}

fn main() -> eframe::Result {
    eframe::run_native(
        "TEMPO2-Rust timing",
        eframe::NativeOptions {
            renderer: eframe::Renderer::Glow,
            viewport: egui::ViewportBuilder::default().with_inner_size([1100., 800.]),
            ..Default::default()
        },
        Box::new(|_| Ok(Box::new(Viewer::new()))),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui_kittest::{Harness, kittest::Queryable};

    fn finish_job(harness: &mut Harness<'_, Viewer>) {
        harness.run_steps(1);
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
        while harness.state().pending.is_some() {
            assert!(
                std::time::Instant::now() < deadline,
                "timing worker did not finish"
            );
            std::thread::sleep(std::time::Duration::from_millis(10));
            harness.run_steps(1);
        }
        harness.run();
    }

    #[test]
    #[ignore = "requires pinned runtime and a graphics adapter for the rendered viewer check"]
    fn load_inspect_and_render() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        let mut viewer = Viewer::new();
        viewer.par = root
            .join("validation/fitting/inputs/f0.par")
            .display()
            .to_string();
        viewer.tim = root
            .join("validation/fitting/inputs/bary_unsorted.tim")
            .display()
            .to_string();
        viewer.runtime = root
            .join("validation/reference/build/source/T2runtime")
            .display()
            .to_string();
        let mut harness = Harness::builder()
            .with_size([1100., 800.])
            .build_eframe(|_| viewer);
        harness.get_by_label("Load and evaluate").click();
        finish_job(&mut harness);
        assert!(
            harness.state().error.is_none(),
            "{:?}",
            harness.state().error
        );
        let result = harness.state().session.as_ref().unwrap().timing();
        assert_eq!(result.pulsars[0].observations.len(), 16);
        let before = serde_json::to_vec(result).unwrap();
        let name = result.pulsars[0].observations[3]
            .barycentric
            .filename
            .text
            .as_ref()
            .unwrap();
        let item = format!("4 · {name}");
        harness.get_by_label("Inspect TOA").click();
        harness.run();
        harness.get_by_label(&item).click();
        harness.run();
        assert_eq!(harness.state().selected, Some(3));
        let plot_id = egui::Id::unique("residuals");
        let plot = egui_plot::PlotMemory::load(&harness.ctx, plot_id).unwrap();
        let full_bounds = *plot.bounds();
        harness.hover_at(plot.transform().frame().center());
        harness.event(egui::Event::Zoom(2.));
        harness.run();
        let zoomed = egui_plot::PlotMemory::load(&harness.ctx, plot_id).unwrap();
        assert!(zoomed.bounds().width() < full_bounds.width());
        assert!(zoomed.bounds().height() < full_bounds.height());
        harness.get_by_label("Reset view").click();
        harness.run();
        assert_eq!(
            *egui_plot::PlotMemory::load(&harness.ctx, plot_id)
                .unwrap()
                .bounds(),
            full_bounds
        );
        let row = &harness.state().rows.as_ref().unwrap()[7];
        let position = egui_plot::PlotMemory::load(&harness.ctx, plot_id)
            .unwrap()
            .transform()
            .position_from_point(&PlotPoint::new(row.sat_mjd, row.residual_us));
        harness.hover_at(position);
        harness.run();
        harness.drag_at(position);
        harness.run_steps(1);
        harness.drop_at(position);
        harness.run();
        assert_eq!(harness.state().selected, Some(7));
        assert_eq!(
            before,
            serde_json::to_vec(harness.state().session.as_ref().unwrap().timing()).unwrap()
        );
        let out = root.join("gui/build/session-viewer-20261104");
        std::fs::create_dir_all(&out).unwrap();
        harness
            .render()
            .unwrap()
            .save(out.join("viewer.png"))
            .unwrap();

        let input_index = harness.state().session.as_ref().unwrap().timing().pulsars[0]
            .observations[7]
            .input_index
            .unwrap();
        harness.get_by_label("Exclude TOA").click();
        finish_job(&mut harness);
        assert!(
            harness.state().error.is_none(),
            "{:?}",
            harness.state().error
        );
        let selected = harness.state().selected.unwrap();
        let row =
            &harness.state().session.as_ref().unwrap().timing().pulsars[0].observations[selected];
        assert_eq!(row.input_index, Some(input_index));
        assert_eq!(row.barycentric.deleted, 1);
        harness
            .render()
            .unwrap()
            .save(out.join("excluded.png"))
            .unwrap();
        harness.get_by_label("Refit").click();
        finish_job(&mut harness);
        assert!(
            harness.state().error.is_none(),
            "{:?}",
            harness.state().error
        );
        let session = harness.state().session.as_ref().unwrap();
        assert!(session.can_save());
        for system in &session.fit_result().unwrap().cycles.last().unwrap().systems {
            assert!(
                system
                    .rows
                    .iter()
                    .all(|row| row.input_index != Some(input_index))
            );
        }
        harness
            .render()
            .unwrap()
            .save(out.join("fitted.png"))
            .unwrap();
        let destination = out.join(format!("saved-{}", std::process::id()));
        harness.get_by_label("New save folder").focus();
        harness.run();
        harness
            .get_by_label("New save folder")
            .type_text(destination.to_str().unwrap());
        harness.run();
        assert_eq!(
            harness.state().save_directory,
            destination.to_str().unwrap()
        );
        let before_save =
            serde_json::to_vec(harness.state().session.as_ref().unwrap().timing()).unwrap();
        harness.get_by_label("Save session").click();
        finish_job(&mut harness);
        assert!(
            harness.state().error.is_none(),
            "{:?}",
            harness.state().error
        );
        assert!(destination.join("session.par").is_file());
        assert!(destination.join("session.tim").is_file());
        assert!(destination.join("session.json").is_file());
        assert_eq!(
            before_save,
            serde_json::to_vec(harness.state().session.as_ref().unwrap().timing()).unwrap()
        );
        harness
            .render()
            .unwrap()
            .save(out.join("saved.png"))
            .unwrap();
        harness.get_by_label("Save session").click();
        finish_job(&mut harness);
        assert!(
            harness
                .state()
                .error
                .as_ref()
                .is_some_and(|error| error.contains("Cannot create"))
        );
        assert_eq!(
            before_save,
            serde_json::to_vec(harness.state().session.as_ref().unwrap().timing()).unwrap()
        );
        harness
            .render()
            .unwrap()
            .save(out.join("save-error.png"))
            .unwrap();
        harness.get_by_label("Include TOA").click();
        finish_job(&mut harness);
        assert!(
            harness.state().error.is_none(),
            "{:?}",
            harness.state().error
        );
        assert!(!harness.state().session.as_ref().unwrap().can_save());
        assert_eq!(
            harness.state().session.as_ref().unwrap().timing().pulsars[0].observations
                [harness.state().selected.unwrap()]
            .barycentric
            .deleted,
            0
        );

        // A failed new load must report the error and clear the prior snapshot.
        harness.state_mut().par = root
            .join("gui/build/missing-input.par")
            .display()
            .to_string();
        harness.get_by_label("Load and evaluate").click();
        finish_job(&mut harness);
        assert!(harness.state().session.is_none());
        assert!(
            harness
                .state()
                .error
                .as_ref()
                .is_some_and(|e| e.contains("unable to read timing model")),
            "{:?}",
            harness.state().error
        );
        harness
            .render()
            .unwrap()
            .save(out.join("input-error.png"))
            .unwrap();
    }
}
