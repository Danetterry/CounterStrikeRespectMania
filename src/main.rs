mod utils;
mod offsets;

// use egui::debug_text::print;
use display_info::DisplayInfo;
use egui_overlay::EguiOverlay;
use egui_render_three_d::{
    three_d::{self, ColorMaterial, Gm, Mesh, Line},
    ThreeDBackend,
};
use egui_render_three_d::three_d::Circle;
use crate::utils::mem_utils::ModuleInfo;
use crate::utils::entity::get_all_entities;
use crate::utils::utils::MemoryReader;

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

    // Starting overlay
    egui_overlay::start(OverlayGui {
        // Passing memory_reader value to use it inside overlay loop
        memory_reader,
        model: None,
    });
}

pub struct OverlayGui {
    pub memory_reader: MemoryReader,
    pub model: Option<Gm<Line, ColorMaterial>>,
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
        glfw_backend.set_window_size([glfw_backend.window.get_size().0 as f32, (display_infos[0].height - 1) as f32]);

        use three_d::*;

        // create model if not yet created
        self.model
            .get_or_insert_with(|| write_on_screen(&three_d_backend.context));

        if let Some(model) = &mut self.model {
            // Create a camera
            let camera = three_d::Camera::new_2d(
                Viewport::new_at_origo(
                    glfw_backend.framebuffer_size_physical[0],
                    glfw_backend.framebuffer_size_physical[1],
                )
            );

            // Get the screen render target to be able to render something on the screen
            egui_render_three_d::three_d::RenderTarget::<'_>::screen(
                &three_d_backend.context,
                glfw_backend.framebuffer_size_physical[0],
                glfw_backend.framebuffer_size_physical[1],
            )
                // Clear the color and depth of the screen render target. use transparent color.
                .clear(ClearState::color_and_depth(0.0, 0.0, 0.0, 0.0, 1.0))
                // Render the triangle with the color material which uses the per vertex colors defined at construction
                .render(&camera, std::iter::once(model), &[]);
        }

        // Creating a vector with all players inside
        let entities = get_all_entities(&self.memory_reader);

        egui::Window::new("Info").show(egui_context, |ui| {
            for entity in entities {
                ui.horizontal(|ui| {
                    ui.label("Name: ");
                    ui.label(format!("{};", entity.name));
                    ui.label("Health: ");
                    ui.label(format!("{};", entity.health));
                    ui.label("Team: ");
                    ui.label(format!("{};", entity.team));
                });
            }
        });

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

fn write_on_screen(three_d_context: &three_d::Context) -> Gm<Line, ColorMaterial> {
    use three_d::*;

    let circle = Gm::new(
        Circle::new(
            three_d_context,
            vec2(500.0, 500.0),
            200.0,
        ),
        ColorMaterial {
            color: Srgba::BLUE,
            ..Default::default()
        },
    );

    let line = Gm::new(
        Line::new(
            three_d_context,
            vec2(500.0, 500.0),
            vec2(800.0, 800.0),
            5.0
        ),
        ColorMaterial {
            color: Srgba::RED,
            ..Default::default()
        },
    );

    line
}