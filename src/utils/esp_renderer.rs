use crate::offsets;
use crate::utils::bones::BoneConnection;
use crate::utils::entity::{get_all_entities, world_to_screen, Bomb, Entity};
use crate::utils::memory_reader::MemoryReader;
use crate::utils::options::CheatOptions;
use egui_render_three_d::three_d::{
    vec2, Camera, ClearState, ColorMaterial, Gm, Line, Matrix4, RenderTarget, Srgba, Vector2,
    Viewport,
};
use egui_render_three_d::{three_d, ThreeDBackend};
use egui_window_glfw_passthrough::GlfwBackend;
use three_d_asset::Vector3;
use three_d_text_builder::{
    Text, TextAlign, TextBuilder, TextMaterial, TextMesh, TextPosition, TextRef,
};

pub fn render_esp(
    three_d_backend: &ThreeDBackend,
    glfw_backend: &GlfwBackend,
    win_size: &[f32; 2],
    memory_reader: &MemoryReader,
    local_player: &Entity,
    text_builder: &mut TextBuilder,
    options: &CheatOptions,
    bone_connection: &[BoneConnection],
    bomb: &Bomb,
) {
    // Getting all players
    let entities = get_all_entities(memory_reader, bone_connection);

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
        local_player,
        options,
        bomb,
    );

    // Render each model in the vector
    for model in lines {
        render_target.render(&camera, std::iter::once(model), &[]);
    }

    if options.text.enabled {
        let texts = get_text(
            &three_d_backend.context,
            win_size,
            text_builder,
            &entities,
            &view_matrix,
            local_player,
            options,
            bomb,
        );

        for model in texts {
            render_target.render(&camera, model, &[]);
        }
    }
}

pub fn get_colours(
    options: &CheatOptions,
    entity: &Entity,
    local_player: &Entity,
) -> Vec<ColorMaterial> {
    let mut colours = vec![];

    if entity.team == local_player.team {
        colours.push(ColorMaterial {
            color: Srgba::from([
                options.line.team_color[0],
                options.line.team_color[1],
                options.line.team_color[2],
                options.line.team_color[3],
            ]),
            ..Default::default()
        });
        colours.push(ColorMaterial {
            color: Srgba::from([
                options.esp_box.team_color[0],
                options.esp_box.team_color[1],
                options.esp_box.team_color[2],
                options.esp_box.team_color[3],
            ]),
            ..Default::default()
        });
        colours.push(ColorMaterial {
            color: Srgba::from([
                options.bones.team_color[0],
                options.bones.team_color[1],
                options.bones.team_color[2],
                options.bones.team_color[3],
            ]),
            ..Default::default()
        });
    } else {
        colours.push(ColorMaterial {
            color: Srgba::from([
                options.line.enemy_color[0],
                options.line.enemy_color[1],
                options.line.enemy_color[2],
                options.line.enemy_color[3],
            ]),
            ..Default::default()
        });
        colours.push(ColorMaterial {
            color: Srgba::from([
                options.esp_box.enemy_color[0],
                options.esp_box.enemy_color[1],
                options.esp_box.enemy_color[2],
                options.esp_box.enemy_color[3],
            ]),
            ..Default::default()
        });
        colours.push(ColorMaterial {
            color: Srgba::from([
                options.bones.enemy_color[0],
                options.bones.enemy_color[1],
                options.bones.enemy_color[2],
                options.bones.enemy_color[3],
            ]),
            ..Default::default()
        });
    }

    colours
}

pub fn get_lines(
    three_d_context: &three_d::Context,
    win_size: &[f32; 2],
    entities: &Vec<Entity>,
    view_matrix: &Matrix4<f32>,
    local_player: &Entity,
    options: &CheatOptions,
    bomb: &Bomb,
) -> Vec<Gm<Line, ColorMaterial>> {
    let mut lines = Vec::new();

    if options.bomb.enabled && bomb.is_planted {
        let bomb_pos = world_to_screen(&bomb.position, view_matrix, win_size);
        let bomb_distance = get_distance(bomb.position, local_player.position);

        let bomb_height = 10.0 - bomb_distance + 5.0;
        let bomb_width = bomb_height * 1.2 + 5.0;

        if bomb_pos != Vector2::new(0.0, 0.0) {
            let bottom = Gm::new(
                Line::new(
                    three_d_context,
                    vec2(bomb_pos.x - bomb_width / 2.0, bomb_pos.y),
                    vec2(bomb_pos.x + bomb_width / 2.0, bomb_pos.y),
                    1.0,
                ),
                ColorMaterial {
                    color: Srgba::from([
                        options.bomb.color[0],
                        options.bomb.color[1],
                        options.bomb.color[2],
                        options.bomb.color[3],
                    ]),
                    ..Default::default()
                },
            );

            lines.push(bottom);

            let upper = Gm::new(
                Line::new(
                    three_d_context,
                    vec2(
                        bomb_pos.x - bomb_width / 2.0,
                        bomb_pos.y - bomb_height,
                    ),
                    vec2(
                        bomb_pos.x + bomb_width / 2.0,
                        bomb_pos.y - bomb_height,
                    ),
                    1.0,
                ),
                ColorMaterial {
                    color: Srgba::from([
                        options.bomb.color[0],
                        options.bomb.color[1],
                        options.bomb.color[2],
                        options.bomb.color[3],
                    ]),
                    ..Default::default()
                },
            );

            lines.push(upper);

            let left = Gm::new(
                Line::new(
                    three_d_context,
                    vec2(
                        bomb_pos.x - bomb_width / 2.0,
                        bomb_pos.y - bomb_height,
                    ),
                    vec2(bomb_pos.x - bomb_width / 2.0, bomb_pos.y),
                    1.0,
                ),
                ColorMaterial {
                    color: Srgba::from([
                        options.bomb.color[0],
                        options.bomb.color[1],
                        options.bomb.color[2],
                        options.bomb.color[3],
                    ]),
                    ..Default::default()
                },
            );

            lines.push(left);

            let right = Gm::new(
                Line::new(
                    three_d_context,
                    vec2(
                        bomb_pos.x + bomb_width / 2.0,
                        bomb_pos.y - bomb_height,
                    ),
                    vec2(bomb_pos.x + bomb_width / 2.0, bomb_pos.y),
                    1.0,
                ),
                ColorMaterial {
                    color: Srgba::from([
                        options.bomb.color[0],
                        options.bomb.color[1],
                        options.bomb.color[2],
                        options.bomb.color[3],
                    ]),
                    ..Default::default()
                },
            );

            lines.push(right);
        }
    }

    for entity in entities {
        // Skip if this is local player
        if entity.pawn == local_player.pawn {
            continue;
        }

        let health_bar_enabled: bool;
        let armor_bar_enabled: bool;

        if entity.team == local_player.team {
            health_bar_enabled = options.health_bar.team_enabled;
            armor_bar_enabled = options.armor_bar.team_enabled;
        } else {
            health_bar_enabled = options.health_bar.enabled;
            armor_bar_enabled = options.armor_bar.enabled;
        }

        // Line = 0, Box = 1, Skeleton = 2
        let colours = get_colours(options, entity, local_player);

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

        if options.line.enabled {
            // Line from screen corner to box bottom
            let line = Gm::new(
                Line::new(
                    three_d_context,
                    vec2(feet.x, feet.y),
                    vec2(win_size[0] / 2.0, 0.0),
                    1.0,
                ),
                colours[0].clone(),
            );

            lines.push(line);
        }

        if options.esp_box.enabled {
            // Lower line of box
            let lower = Gm::new(
                Line::new(
                    three_d_context,
                    vec2(feet.x + width, feet.y),
                    vec2(feet.x - width, feet.y),
                    1.0,
                ),
                colours[1].clone(),
            );

            lines.push(lower);

            // Upper line of box
            let upper = Gm::new(
                Line::new(
                    three_d_context,
                    vec2(
                        feet.x + width,
                        head.y - ((feet.y - head.y) / 6.5),
                    ),
                    vec2(
                        feet.x - width,
                        head.y - ((feet.y - head.y) / 6.5),
                    ),
                    1.0,
                ),
                colours[1].clone(),
            );

            lines.push(upper);

            // Lefty line of box
            let left = Gm::new(
                Line::new(
                    three_d_context,
                    vec2(
                        feet.x - width,
                        head.y - ((feet.y - head.y) / 6.5),
                    ),
                    vec2(feet.x - width, feet.y),
                    1.0,
                ),
                colours[1].clone(),
            );

            lines.push(left);

            // Righty line of box
            let right = Gm::new(
                Line::new(
                    three_d_context,
                    vec2(
                        feet.x + width,
                        head.y - ((feet.y - head.y) / 6.5),
                    ),
                    vec2(feet.x + width, feet.y),
                    1.0,
                ),
                colours[1].clone(),
            );

            lines.push(right);
        }

        if options.bones.enabled {
            for bone in &entity.bones {
                let first_bone = world_to_screen(&bone.0, view_matrix, win_size);
                let second_bone = world_to_screen(&bone.1, view_matrix, win_size);

                if first_bone == Vector2::new(0.0, 0.0) {
                    continue;
                }

                if second_bone == Vector2::new(0.0, 0.0) {
                    continue;
                }

                let bone_line = Gm::new(
                    Line::new(
                        three_d_context,
                        vec2(first_bone.x, first_bone.y),
                        vec2(second_bone.x, second_bone.y),
                        1.0,
                    ),
                    colours[2].clone(),
                );

                lines.push(bone_line);
            }
        }

        // Calculate thickness for health bar from box width
        let mut thickness = width / 5.0;

        // Thickness limits
        thickness = thickness.clamp(2.0, 5.0);

        if armor_bar_enabled {
            // Calculate health multiplier (for example 64 hp -> 0.64)
            let armor_multiplier = entity.armor as f32 / 100.0;

            // Armor bar renderer
            let armor_bar = Gm::new(
                Line::new(
                    three_d_context,
                    vec2(
                        feet.x + width - thickness * 2.0 - (thickness / 2.0),
                        head.y
                            - ((feet.y - head.y) / 6.5)
                            - (head.y - feet.y - ((feet.y - head.y) / 6.5))
                                * (1.0 - armor_multiplier),
                    ),
                    vec2(
                        feet.x + width - thickness * 2.0 - (thickness / 2.0),
                        feet.y,
                    ),
                    thickness,
                ),
                ColorMaterial {
                    color: Srgba::new(0, 0, (armor_multiplier * 255.0).round() as u8, 100),
                    ..Default::default()
                },
            );

            lines.push(armor_bar);
        }

        if health_bar_enabled {
            // Calculate health multiplier (for example 64 hp -> 0.64)
            let health_multiplier = entity.health as f32 / 100.0;

            // Health bar renderer
            let health_bar = Gm::new(
                Line::new(
                    three_d_context,
                    vec2(
                        feet.x + width - thickness,
                        head.y
                            - ((feet.y - head.y) / 6.5)
                            - (head.y - feet.y - ((feet.y - head.y) / 6.5))
                                * (1.0 - health_multiplier),
                    ),
                    vec2(feet.x + width - thickness, feet.y),
                    thickness,
                ),
                ColorMaterial {
                    color: Srgba::new(
                        ((1.0 - health_multiplier) * 255.0).round() as u8,
                        (health_multiplier * 255.0).round() as u8,
                        0,
                        100,
                    ),
                    ..Default::default()
                },
            );

            lines.push(health_bar);
        }
    }

    lines
}

fn get_text<'a>(
    three_d_context: &'a three_d::Context,
    win_size: &'a [f32; 2],
    text_builder: &'a mut TextBuilder,
    entities: &Vec<Entity>,
    view_matrix: &Matrix4<f32>,
    local_player: &Entity,
    options: &CheatOptions,
    bomb: &Bomb,
) -> Vec<impl Iterator<Item = Gm<TextMesh, TextMaterial>> + 'a> {
    // Init text vector
    let mut texts = Vec::new();

    if options.bomb.enabled && bomb.is_planted {
        let bomb_pos = world_to_screen(&bomb.position, view_matrix, win_size);
        let bomb_distance = get_distance(bomb.position, local_player.position);

        let bomb_height = 10.0 - bomb_distance + 5.0;

        let c4_text = Text {
            // Entity name as text to render
            text: "C4".to_string(),
            // Idk but this is necessary
            align: TextAlign::Viewport(0, 1),
            // Move text to box corner
            position: TextPosition::Pixels(vec2(
                bomb_pos.x,
                bomb_pos.y + 10.0,
            )),
            // Set color
            color: Srgba::WHITE,
            // Set text size based on box width
            // size: width / 3.0,
            // Using fixed size because of memory leak
            size: 10.0,
            ..Default::default()
        };

        let c4_text_model = text_builder.build(
            three_d_context,
            &[
                TextRef {
                    text: "",
                    ..Default::default()
                },
                c4_text.as_ref(),
            ],
        );

        texts.push(c4_text_model);
    }

    for entity in entities {
        // Skip if this is local player
        if entity.pawn == local_player.pawn {
            continue;
        }

        let text_color: Srgba;

        if entity.team == local_player.team {
            text_color = Srgba::from([
                options.text.team_color[0],
                options.text.team_color[1],
                options.text.team_color[2],
                options.text.team_color[3],
            ]);
        } else {
            text_color = Srgba::from([
                options.text.enemy_color[0],
                options.text.enemy_color[1],
                options.text.enemy_color[2],
                options.text.enemy_color[3],
            ]);
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
            position: TextPosition::Pixels(vec2(feet.x, feet.y - 5.0)),
            // Set colour
            color: text_color,
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
                head.y + 10.0 - ((feet.y - head.y) / 6.5),
            )),
            // Set color
            color: text_color,
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

fn get_distance(vec: Vector3<f32>, local_vec: Vector3<f32>) -> f32 {
    let dx = vec.x - local_vec.x;
    let dy = vec.y - local_vec.y;
    let dz = vec.z - local_vec.z;

    f32::sqrt(dx * dx + dy * dy + dz * dz) / 100.0
}
