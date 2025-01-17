use crate::offsets;
use crate::utils::bones::BoneConnection;
use crate::utils::entity::{get_all_entities, get_local_player, world_to_screen, Entity};
use crate::utils::memory_reader::MemoryReader;
use crate::utils::options::CheatOptions;
use egui_render_three_d::three_d::{
    vec2, Camera, ClearState, ColorMaterial, Gm, Line, Matrix4, RenderTarget, Srgba, Vector2,
    Viewport,
};
use egui_render_three_d::{three_d, ThreeDBackend};
use egui_window_glfw_passthrough::GlfwBackend;
use three_d_text_builder::{
    Text, TextAlign, TextBuilder, TextMaterial, TextMesh, TextPosition, TextRef,
};

pub fn render_esp(
    three_d_backend: &ThreeDBackend,
    glfw_backend: &GlfwBackend,
    win_size: &[f32; 2],
    memory_reader: &MemoryReader,
    text_builder: &mut TextBuilder,
    options: &CheatOptions,
    bone_connection: &[BoneConnection],
) {
    // Getting all players
    let entities = get_all_entities(memory_reader, bone_connection);

    // Getting local player for team compare
    let local_player = get_local_player(memory_reader, bone_connection);

    // This needed for esp
    let view_matrix = memory_reader.read_matrix4_f32(
        memory_reader.module + offsets::offsets::cs2_dumper::offsets::client_dll::dwViewMatrix,
    );

    // Creating camera
    let camera = Camera::new_2d(Viewport::new_at_origo(
        glfw_backend.framebuffer_size_physical[0],
        glfw_backend.framebuffer_size_physical[1],
    ));

    // Get the screen render target
    let render_target = RenderTarget::<'_>::screen(
        &three_d_backend.context,
        glfw_backend.framebuffer_size_physical[0],
        glfw_backend.framebuffer_size_physical[1],
    );

    // Clear the color and depth of the screen render target
    render_target.clear(ClearState::color_and_depth(0.0, 0.0, 0.0, 0.0, 1.0));

    // Getting all lines
    let lines = get_lines(
        &three_d_backend.context,
        win_size,
        &entities,
        &view_matrix,
        local_player.team,
        options,
    );

    // Render each model in the vector
    for model in lines {
        render_target.render(&camera, std::iter::once(model), &[]);
    }

    if options.enable_text {
        let texts = get_text(
            &three_d_backend.context,
            win_size,
            text_builder,
            &entities,
            &view_matrix,
            local_player.team,
        );

        for model in texts {
            render_target.render(&camera, model, &[]);
        }
    }
}

pub fn get_lines(
    three_d_context: &three_d::Context,
    win_size: &[f32; 2],
    entities: &Vec<Entity>,
    view_matrix: &Matrix4<f32>,
    local_player_team: i32,
    options: &CheatOptions,
) -> Vec<Gm<Line, ColorMaterial>> {
    let mut lines = Vec::new();

    for entity in entities {
        // Skip local player's team
        if entity.team == local_player_team {
            continue;
        }

        // Translate player position into 2D
        let head = world_to_screen(&entity.eye_position, view_matrix, win_size);
        let feet = world_to_screen(&entity.position, view_matrix, win_size);

        // Skip if not on screen
        if feet == Vector2::new(0.0, 0.0) || head == Vector2::new(0.0, 0.0) {
            continue;
        }

        // Skip if died / spectator
        if entity.life_state != 0 {
            continue;
        }

        // Calculate box width
        let width = (feet.y - head.y) / 3.0;

        if options.enable_line {
            // Line from screen corner to box bottom
            let line = Gm::new(
                Line::new(
                    three_d_context,
                    vec2(feet.x, win_size[1] - feet.y),
                    vec2(win_size[0] / 2.0, 0.0),
                    1.0,
                ),
                ColorMaterial {
                    color: Srgba::GREEN,
                    ..Default::default()
                },
            );

            lines.push(line);
        }

        if !options.enable_box {
            continue;
        }

        // Lower line of box
        let lower = Gm::new(
            Line::new(
                three_d_context,
                vec2(feet.x + width, win_size[1] - feet.y),
                vec2(feet.x - width, win_size[1] - feet.y),
                1.0,
            ),
            ColorMaterial {
                color: Srgba::GREEN,
                ..Default::default()
            },
        );

        lines.push(lower);

        // Upper line of box
        let upper = Gm::new(
            Line::new(
                three_d_context,
                vec2(
                    feet.x + width,
                    win_size[1] - head.y + ((feet.y - head.y) / 6.5),
                ),
                vec2(
                    feet.x - width,
                    win_size[1] - head.y + ((feet.y - head.y) / 6.5),
                ),
                1.0,
            ),
            ColorMaterial {
                color: Srgba::GREEN,
                ..Default::default()
            },
        );

        lines.push(upper);

        // Lefty line of box
        let left = Gm::new(
            Line::new(
                three_d_context,
                vec2(
                    feet.x - width,
                    win_size[1] - head.y + ((feet.y - head.y) / 6.5),
                ),
                vec2(feet.x - width, win_size[1] - feet.y),
                1.0,
            ),
            ColorMaterial {
                color: Srgba::GREEN,
                ..Default::default()
            },
        );

        lines.push(left);

        // Righty line of box
        let right = Gm::new(
            Line::new(
                three_d_context,
                vec2(
                    feet.x + width,
                    win_size[1] - head.y + ((feet.y - head.y) / 6.5),
                ),
                vec2(feet.x + width, win_size[1] - feet.y),
                1.0,
            ),
            ColorMaterial {
                color: Srgba::GREEN,
                ..Default::default()
            },
        );

        lines.push(right);

        for bone in &entity.bones {
            let first_bone = world_to_screen(&bone.0, view_matrix, win_size);
            let second_bone = world_to_screen(&bone.1, view_matrix, win_size);

            let bone_line = Gm::new(
                Line::new(
                    three_d_context,
                    vec2(first_bone.x, win_size[1] - first_bone.y),
                    vec2(second_bone.x, win_size[1] - second_bone.y),
                    1.0,
                ),
                ColorMaterial {
                    color: Srgba::GREEN,
                    ..Default::default()
                },
            );

            lines.push(bone_line);
        }

        // Calculate health multiplier (for example 64 hp -> 0.64)
        let health_multiplier = entity.health as f32 / 100.0;

        // Calculate thickness for health bar from box width
        let mut thickness = width / 5.0;

        // Thickness limits
        thickness = thickness.clamp(2.0, 5.0);

        // Health bar renderer
        let health_bar = Gm::new(
            Line::new(
                three_d_context,
                vec2(
                    feet.x - width - thickness,
                    win_size[1] - head.y
                        + ((feet.y - head.y) / 6.5)
                        + (head.y - feet.y - ((feet.y - head.y) / 6.5)) * (1.0 - health_multiplier),
                ),
                vec2(feet.x - width - thickness, win_size[1] - feet.y),
                thickness,
            ),
            ColorMaterial {
                color: Srgba::new(
                    ((1.0 - health_multiplier) * 255.0).round() as u8,
                    (health_multiplier * 255.0).round() as u8,
                    0,
                    0,
                ),
                ..Default::default()
            },
        );

        lines.push(health_bar);
    }

    lines
}

pub fn get_text<'a>(
    three_d_context: &'a three_d::Context,
    win_size: &'a [f32; 2],
    text_builder: &'a mut TextBuilder,
    entities: &Vec<Entity>,
    view_matrix: &Matrix4<f32>,
    local_player_team: i32,
) -> Vec<impl Iterator<Item = Gm<TextMesh, TextMaterial>> + 'a> {
    // Init text vector
    let mut texts = Vec::new();

    for entity in entities {
        // Skip local player's team
        if entity.team == local_player_team {
            continue;
        }

        // Translate player position into 2D
        let head = world_to_screen(&entity.eye_position, view_matrix, win_size);
        let feet = world_to_screen(&entity.position, view_matrix, win_size);

        // Skip if not on screen
        if head == Vector2::new(0.0, 0.0) {
            continue;
        }

        // Skip if died / spectator
        if entity.life_state != 0 {
            continue;
        }

        let name_text = Text {
            // Entity name as text to render
            text: entity.name.to_string(),
            // Idk but this is necessary
            align: TextAlign::Viewport(0, 0),
            // Move text to box corner
            position: TextPosition::Pixels(vec2(feet.x, win_size[1] - feet.y - 5.0)),
            // Set text size based on box width
            // size: width / 3.0,
            // Using fixed size because of memory leak
            size: 10.0,
            ..Default::default()
        };

        let name_text_model = text_builder.build(
            three_d_context,
            &[
                TextRef {
                    text: "",
                    ..Default::default()
                },
                name_text.as_ref(),
            ],
        );

        texts.push(name_text_model);

        drop(name_text);

        let weapon_text = Text {
            // Entity name as text to render
            text: entity.active_weapon_name.to_string(),
            // Idk but this is necessary
            align: TextAlign::Viewport(0, 1),
            // Move text to box corner
            position: TextPosition::Pixels(vec2(
                feet.x,
                win_size[1] - head.y + 10.0 + ((feet.y - head.y) / 6.5),
            )),
            // Set text size based on box width
            // size: width / 3.0,
            // Using fixed size because of memory leak
            size: 10.0,
            ..Default::default()
        };

        let weapon_text_model = text_builder.build(
            three_d_context,
            &[
                TextRef {
                    text: "",
                    ..Default::default()
                },
                weapon_text.as_ref(),
            ],
        );

        texts.push(weapon_text_model);

        drop(weapon_text);
    }

    texts
}
