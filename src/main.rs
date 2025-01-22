mod offsets;
mod utils;

// use egui::debug_text::print;
use crate::utils::bones::BoneConnection;
use crate::utils::esp_renderer::render_esp;
use crate::utils::memory_reader::MemoryReader;
use crate::utils::options::CheatOptions;
use display_info::DisplayInfo;
use egui::{pos2, Color32, Pos2, Shadow};
use egui_overlay::EguiOverlay;
use egui_render_three_d::ThreeDBackend;
use three_d_asset::io::load;
use three_d_text_builder::{TextBuilder, TextBuilderSettings};
use crate::utils::bhop::perform_bunny_hop;
use crate::utils::entity::{get_bomb, get_local_player};
use enigo::{
    Enigo, Settings,
};

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

    // Creating bones connection vector list
    let bones_connection = BoneConnection::get();
    
    // Enigo for bhop
    let enigo = Enigo::new(&Settings::default()).unwrap();

    // Starting overlay
    egui_overlay::start(OverlayGui {
        // Passing memory_reader value to use it inside overlay loop
        memory_reader,
        text_builder,
        options,
        bones_connection,
        enigo
    });
}

pub struct OverlayGui {
    pub memory_reader: MemoryReader,
    pub text_builder: TextBuilder,
    pub options: CheatOptions,
    pub bones_connection: Vec<BoneConnection>,
    pub enigo: Enigo,
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

        egui_context.set_visuals_of(egui::Theme::Dark, egui::Visuals { ..Default::default() });

        // All menus
        egui::Window::new("CounterStrikeRespectMania").show(egui_context, |ui| {
            ui.checkbox(&mut self.options.line.enabled, "Enable line");
            ui.collapsing("Line options", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Enemy colour");
                    ui.color_edit_button_srgba(&mut self.options.line.enemy_color)
                });
                ui.horizontal(|ui| {
                    ui.label("Team colour");
                    ui.color_edit_button_srgba(&mut self.options.line.team_color)
                });
            });

            ui.checkbox(&mut self.options.esp_box.enabled, "Enable box");
            ui.collapsing("Box options", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Enemy colour");
                    ui.color_edit_button_srgba(&mut self.options.esp_box.enemy_color)
                });
                ui.horizontal(|ui| {
                    ui.label("Team colour");
                    ui.color_edit_button_srgba(&mut self.options.esp_box.team_color)
                });
            });

            ui.checkbox(&mut self.options.health_bar.enabled, "Enable health bar");
            ui.checkbox(&mut self.options.health_bar.team_enabled, "Health bar on team");

            ui.checkbox(&mut self.options.armor_bar.enabled, "Enable armor bar");
            ui.checkbox(&mut self.options.armor_bar.team_enabled, "Armor bar on team");

            ui.checkbox(&mut self.options.bones.enabled, "Enable bones");
            ui.collapsing("Bones options", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Enemy colour");
                    ui.color_edit_button_srgba(&mut self.options.bones.enemy_color)
                });
                ui.horizontal(|ui| {
                    ui.label("Team colour");
                    ui.color_edit_button_srgba(&mut self.options.bones.team_color)
                });
            });

            ui.checkbox(&mut self.options.text.enabled, "Enable text");
            ui.collapsing("Text options", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Enemy colour");
                    ui.color_edit_button_srgba(&mut self.options.text.enemy_color)
                });
                ui.horizontal(|ui| {
                    ui.label("Team colour");
                    ui.color_edit_button_srgba(&mut self.options.text.team_color)
                });
            });
            
            ui.checkbox(&mut self.options.bomb.enabled, "Enable bomb esp");
            ui.horizontal(|ui| {
                ui.label("Bomb ESP Colour");
                ui.color_edit_button_srgba(&mut self.options.bomb.color)
            });

            ui.separator();

            ui.checkbox(&mut self.options.bunny_hop.enabled, "Enable bunny hop");

            ui.separator();

            ui.checkbox(&mut self.options.bomb_timer.enabled, "Enable bomb timer");
            ui.checkbox(&mut self.options.bomb_timer.resizable, "Enable bomb timer resizable");
            ui.add(egui::Slider::new(&mut self.options.bomb_timer.y_offset, 0.0..=win_size[1]).text("px"));
        });

        egui_context.set_visuals_of(egui::Theme::Dark, egui::Visuals { window_fill: Color32::from_rgba_unmultiplied(27, 27, 27, 150), window_shadow: Shadow::NONE, override_text_color: Some(Color32::WHITE), ..Default::default() });

        let bomb = get_bomb(&self.memory_reader);

        egui::Window::new("Bomb Timer").title_bar(false).resizable(self.options.bomb_timer.resizable).open(&mut self.options.bomb_timer.enabled).pivot(egui::Align2::CENTER_TOP).fixed_pos(pos2(win_size[0] / 2.0, self.options.bomb_timer.y_offset)).show(egui_context, |ui| {
            ui.style_mut().interaction.selectable_labels = false;
            
            ui.vertical_centered(|ui| {
                if !bomb.is_planted {
                    ui.label("Bomb is not planted");
                }
                else {
                    ui.colored_label(Color32::from_rgb(220, 0, 0), "Bomb planted");
                    if bomb.site == 0 {
                        ui.label("Bomb site is A");
                    }
                    else if bomb.site == 1 {
                        ui.label("Bomb site is B");
                    }
                    ui.label(format!("Time left: {:.2}", -bomb.time));
                }
            });
        });

        // Getting local player
        let local_player = get_local_player(&self.memory_reader, &self.bones_connection);

        // Rendering ESP
        render_esp(
            three_d_backend,
            glfw_backend,
            &win_size,
            &self.memory_reader,
            &local_player,
            &mut self.text_builder,
            &self.options,
            &self.bones_connection,
            &bomb,
        );
        
        // Bunny hop
        perform_bunny_hop(&local_player, &self.memory_reader, &mut self.options, &mut self.enigo);

        // here you decide if you want to be passthrough or not.
        if egui_context.wants_pointer_input() || egui_context.wants_keyboard_input() {
            // we need input, so we need the window to be NOT passthrough
            glfw_backend.set_passthrough(false);
        } else {
            // we don't care about input, so the window can be passthrough now
            glfw_backend.set_passthrough(true)
        }
        egui_context.request_repaint();

        // std::thread::sleep(std::time::Duration::from_millis(1));
    }
}
