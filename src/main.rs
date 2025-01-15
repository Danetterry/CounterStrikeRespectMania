mod offsets;
mod utils;

// use egui::debug_text::print;
use crate::utils::esp_renderer::render_esp;
use crate::utils::memory_reader::MemoryReader;
use crate::utils::options::CheatOptions;
use display_info::DisplayInfo;
use egui_overlay::EguiOverlay;
use egui_render_three_d::ThreeDBackend;
use three_d_asset::io::load;
use three_d_text_builder::{TextBuilder, TextBuilderSettings};

fn main() {
    // This is needed for logs
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};
    // if RUST_LOG is not set, we will use the following filters
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or(EnvFilter::new("debug,wgpu=warn,naga=warn")),
        )
        .init();

    // Initializing MemoryReading
    let memory_reader = MemoryReader::new("cs2.exe", "client.dll");

    // Load font for text builder
    let assets = load(&["C:/Windows/Fonts/Arial.ttf"])
        .expect("Failed to load font (C:/Windows/Fonts/Arial.ttf)");

    let text_builder = TextBuilder::new(
        assets.get("Arial.ttf").unwrap(),
        TextBuilderSettings::default(),
    )
    .expect("Failed to create text builder from TTF font");

    // Creating options with default settings
    let options = CheatOptions::default();

    // Starting overlay
    egui_overlay::start(OverlayGui {
        // Passing memory_reader value to use it inside overlay loop
        memory_reader,
        text_builder,
        options,
    });
}

pub struct OverlayGui {
    pub memory_reader: MemoryReader,
    pub text_builder: TextBuilder,
    pub options: CheatOptions,
}

impl EguiOverlay for OverlayGui {
    fn gui_run(
        // I just take it from example https://github.com/coderedart/egui_overlay/blob/master/examples/triangle/src/main.rs
        &mut self,
        egui_context: &egui::Context,
        three_d_backend: &mut ThreeDBackend,
        glfw_backend: &mut egui_window_glfw_passthrough::GlfwBackend,
    ) {
        // Things for set resolution
        // Getting all displays
        let display_infos = DisplayInfo::all().unwrap();
        // Maximizing the window
        glfw_backend.window.maximize();
        glfw_backend.window.set_decorated(false);
        // I needed to remove 1 pixel from the height because it made the screen black
        let win_size = [
            glfw_backend.window.get_size().0 as f32,
            (display_infos[0].height - 1) as f32,
        ];
        glfw_backend.set_window_size(win_size);

        // All menus
        egui::Window::new("CounterStrikeRespectMania").show(egui_context, |ui| {
            ui.checkbox(&mut self.options.enable_line, "Включить линию снизу экрана");
            ui.checkbox(&mut self.options.enable_box, "Включить боксы");
            ui.checkbox(&mut self.options.enable_text, "Включить текст");
        });

        // Rendering ESP
        render_esp(
            three_d_backend,
            glfw_backend,
            &win_size,
            &self.memory_reader,
            &mut self.text_builder,
            &self.options,
        );

        // here you decide if you want to be passthrough or not.
        if egui_context.wants_pointer_input() || egui_context.wants_keyboard_input() {
            // we need input, so we need the window to be NOT passthrough
            glfw_backend.set_passthrough(false);
        } else {
            // we don't care about input, so the window can be passthrough now
            glfw_backend.set_passthrough(true)
        }
        egui_context.request_repaint();
    }
}
