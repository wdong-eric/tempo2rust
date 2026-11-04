//! Linux X11/OpenGL lifecycle check; the existing kittest covers widget actions.
use super::*;
use eframe::glow::HasContext;
use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};
use winit::platform::x11::EventLoopBuilderExtX11;

struct NativeCheck {
    viewer: Viewer,
    output: PathBuf,
    deadline: Instant,
    requested: bool,
    completed: Arc<AtomicBool>,
}

impl eframe::App for NativeCheck {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        assert!(
            Instant::now() < self.deadline,
            "native viewer check timed out"
        );
        eframe::App::ui(&mut self.viewer, ui, frame);
        if self.viewer.pending.is_none() && !self.requested {
            assert!(self.viewer.error.is_none(), "{:?}", self.viewer.error);
            let session = self.viewer.session.as_ref().expect("loaded native session");
            assert_eq!(session.timing().pulsars[0].observations.len(), 16);
            assert_eq!(self.viewer.rows.as_ref().unwrap().len(), 16);
            let mut timing = serde_json::to_vec_pretty(session.timing()).unwrap();
            timing.push(b'\n');
            std::fs::write(self.output.join("native-timing.json"), timing).unwrap();
            let output = self.output.join("native.png");
            let completed = self.completed.clone();
            let ctx = ui.ctx().clone();
            ui.ctx().request_screenshot(move |pixels| {
                let rgba: Vec<_> = pixels.pixels.iter().flat_map(|p| p.to_array()).collect();
                image::save_buffer(
                    output,
                    &rgba,
                    pixels.size[0] as u32,
                    pixels.size[1] as u32,
                    image::ColorType::Rgba8,
                )
                .unwrap();
                completed.store(true, Ordering::SeqCst);
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            });
            self.requested = true;
        }
        ui.ctx().request_repaint_after(Duration::from_millis(16));
    }
}

#[test]
#[ignore = "requires Linux X11, an OpenGL driver, TEMPO2 and a fresh TEMPO2_GUI_TEST_OUTPUT"]
fn native_x11_load_render_close() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let output = PathBuf::from(std::env::var_os("TEMPO2_GUI_TEST_OUTPUT").unwrap());
    std::fs::create_dir_all(output.parent().unwrap()).unwrap();
    std::fs::create_dir(&output).expect("native evidence directory must be fresh");
    let completed = Arc::new(AtomicBool::new(false));
    let observed = completed.clone();
    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Glow,
        viewport: egui::ViewportBuilder::default().with_inner_size([1100., 800.]),
        // Rust tests run on a worker thread; winit explicitly supports this on X11.
        event_loop_builder: Some(Box::new(|builder| {
            builder.with_x11().with_any_thread(true);
        })),
        ..Default::default()
    };
    eframe::run_native(
        "TEMPO2-Rust native qualification",
        options,
        Box::new(move |cc| {
            let gl = cc.gl.as_ref().expect("native OpenGL context");
            // eframe supplies a live, current context during app creation.
            let renderer = unsafe {
                format!(
                    "{}\n{}\n{}\n",
                    gl.get_parameter_string(eframe::glow::VERSION),
                    gl.get_parameter_string(eframe::glow::VENDOR),
                    gl.get_parameter_string(eframe::glow::RENDERER)
                )
            };
            std::fs::write(output.join("opengl.txt"), renderer).unwrap();
            let mut viewer = Viewer::new();
            viewer.par = root
                .join("validation/fitting/inputs/f0.par")
                .display()
                .to_string();
            viewer.tim = root
                .join("validation/fitting/inputs/bary_unsorted.tim")
                .display()
                .to_string();
            viewer.runtime = std::env::var("TEMPO2").expect("explicit pinned runtime");
            viewer.start(Action::Load, &cc.egui_ctx);
            Ok(Box::new(NativeCheck {
                viewer,
                output,
                deadline: Instant::now() + Duration::from_secs(30),
                requested: false,
                completed,
            }))
        }),
    )
    .unwrap();
    assert!(
        observed.load(Ordering::SeqCst),
        "native frame was not captured"
    );
}
