use eframe::egui::{self, Color32};
use egui_plot::{Line, MarkerShape, Plot, PlotPoint, Points};
use std::{
    path::Path,
    sync::mpsc::{self, Receiver, TryRecvError},
};
use tempo2_rust::{bats::Number, timing::TimingResult};

const ACTIVE: Color32 = Color32::from_rgb(72, 160, 225);
const EXCLUDED: Color32 = Color32::from_rgb(231, 153, 69);

struct Viewer {
    par: String,
    tim: String,
    runtime: String,
    pending: Option<Receiver<Result<TimingResult, String>>>,
    result: Option<TimingResult>,
    rows: Result<Vec<tempo2_gui::PlotToa>, String>,
    selected: Option<usize>,
    error: Option<String>,
    reset_plot: bool,
}

impl Viewer {
    fn new() -> Self {
        Self {
            par: String::new(),
            tim: String::new(),
            runtime: std::env::var("TEMPO2").unwrap_or_default(),
            pending: None,
            result: None,
            rows: Ok(vec![]),
            selected: None,
            error: None,
            reset_plot: false,
        }
    }

    fn start_load(&mut self, ctx: &egui::Context) {
        let (par, tim, runtime) = (self.par.clone(), self.tim.clone(), self.runtime.clone());
        let (sender, receiver) = mpsc::channel();
        let ctx = ctx.clone();
        self.pending = Some(receiver);
        self.result = None;
        self.selected = None;
        self.error = None;
        std::thread::spawn(move || {
            let result = tempo2_gui::load(Path::new(&par), Path::new(&tim), Path::new(&runtime))
                .map_err(|e| e.to_string());
            let _ = sender.send(result); // The user may have closed the window.
            ctx.request_repaint();
        });
    }

    fn receive(&mut self) {
        let received = match self.pending.as_ref().map(Receiver::try_recv) {
            Some(Ok(result)) => result,
            Some(Err(TryRecvError::Disconnected)) => {
                Err("Timing worker stopped before returning a result".into())
            }
            _ => return,
        };
        self.pending = None;
        match received {
            Ok(result) => {
                self.rows = tempo2_gui::plot_toas(&result.pulsars[0]);
                self.selected = (!result.pulsars[0].observations.is_empty()).then_some(0);
                self.result = Some(result);
                self.reset_plot = true;
            }
            Err(error) => self.error = Some(error),
        }
    }
}

impl eframe::App for Viewer {
    fn ui(&mut self, ui: &mut egui::Ui, _: &mut eframe::Frame) {
        self.receive();
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("TEMPO2-Rust · Timing viewer");
            ui.label("Load PAR/TIM, inspect residuals and arrival times. Evaluation removes the residual mean; it does not fit.");
            ui.add_enabled_ui(self.pending.is_none(), |ui| {
                egui::Grid::new("inputs").num_columns(2).show(ui, |ui| {
                    for (label, value) in [("PAR file", &mut self.par), ("TIM file", &mut self.tim), ("TEMPO2 resources", &mut self.runtime)] {
                        let label = ui.label(label);
                        ui.add(egui::TextEdit::singleline(value).desired_width(f32::INFINITY)).labelled_by(label.id);
                        ui.end_row();
                    }
                });
                if ui.button("Load and evaluate").clicked() { self.start_load(ui.ctx()); }
            });
            if self.pending.is_some() { ui.horizontal(|ui| { ui.spinner(); ui.label("Computing timing results…"); }); }
            if let Some(error) = &self.error { ui.colored_label(Color32::LIGHT_RED, error); }
            let Some(result) = &self.result else { return; };
            let pulsar = &result.pulsars[0];
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
                ui.collapsing("Loaded inputs and resources", |ui| {
                    for input in &result.inputs { ui.label(format!("{} · {}", input.role, input.path.text.as_deref().unwrap_or("Non-UTF8 path"))); }
                    ui.label(format!("{} consumed resource records", result.resources.len()));
                });
            });
        });
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
        assert!(
            harness.state().error.is_none(),
            "{:?}",
            harness.state().error
        );
        let result = harness.state().result.as_ref().unwrap();
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
            serde_json::to_vec(harness.state().result.as_ref().unwrap()).unwrap()
        );
        let out = root.join("gui/build/first-viewer-20261104");
        std::fs::create_dir_all(&out).unwrap();
        harness
            .render()
            .unwrap()
            .save(out.join("viewer.png"))
            .unwrap();

        // A failed new load must report the error and clear the prior snapshot.
        harness.state_mut().par = root
            .join("gui/build/missing-input.par")
            .display()
            .to_string();
        harness.get_by_label("Load and evaluate").click();
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
        assert!(harness.state().result.is_none());
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
