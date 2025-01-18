use crate::offsets::client_dll::cs2_dumper::schemas::client_dll;
use crate::offsets::offsets::cs2_dumper::offsets::client_dll as client_dll_offsets;
use crate::utils::bones::BoneConnection;
use crate::utils::memory_reader::MemoryReader;
use crate::utils::weapon::{proccess_weapon, Weapon};
use egui_render_three_d::three_d::{Matrix4, Vector2, Vector3};
use std::sync::mpsc;
use std::thread;

// This structure contains all needed thing about player
pub struct Entity {
    pub name: String,
    pub pawn: usize,
    pub health: i32,
    pub armor: i32,
    pub team: i32,
    pub position: Vector3<f32>,
    pub eye_position: Vector3<f32>,
    pub life_state: u8,
    //    pub active_weapon_index: i16,
    pub active_weapon_name: String,
    pub bones: Vec<(Vector3<f32>, Vector3<f32>)>,
}

pub fn get_all_entities(
    memory_reader: &MemoryReader,
    bone_connection: &[BoneConnection],
) -> Vec<Entity> {
    // Creating vector with entities
    let mut entities: Vec<Entity> = Vec::new();

    // This loop was stolen from https://www.unknowncheats.me/forum/3907391-post4.html
    for i in 0..64 {
        let entity =
            memory_reader.read_usize(memory_reader.module + client_dll_offsets::dwEntityList);

        let mut list_entity = memory_reader.read_usize(entity + (((8 * (i & 0x7FFF)) >> 9) + 16));
        if list_entity == 0 {
            continue;
        }

        let entity_controller = memory_reader.read_usize(list_entity + (120) * (i & 0x1FF));
        if entity_controller == 0 {
            continue;
        }

        let entity_controller_pawn = memory_reader
            .read_usize(entity_controller + client_dll::CCSPlayerController::m_hPlayerPawn);
        if entity_controller_pawn == 0 {
            continue;
        }

        list_entity = memory_reader
            .read_usize(entity + (0x8 * ((entity_controller_pawn & 0x7FFF) >> 9) + 16));

        let entity_pawn =
            memory_reader.read_usize(list_entity + (120) * (entity_controller_pawn & 0x1FF));

        let player_position =
            memory_reader.read_vec_f32(entity_pawn + client_dll::C_BasePlayerPawn::m_vOldOrigin);
        if player_position == Vector3::new(0.0, 0.0, 0.0) {
            continue;
        }

        let player_team =
            memory_reader.read_i32(entity_pawn + client_dll::C_BaseEntity::m_iTeamNum);

        let player_eye_position = player_position
            + memory_reader
                .read_vec_f32(entity_pawn + client_dll::C_BaseModelEntity::m_vecViewOffset);

        let player_health =
            memory_reader.read_i32(entity_pawn + client_dll::C_BaseEntity::m_iHealth);

        let player_armor =
            memory_reader.read_i32(entity_pawn + client_dll::C_CSPlayerPawn::m_ArmorValue);

        let player_name = memory_reader
            .read_string(entity_controller + client_dll::CBasePlayerController::m_iszPlayerName);

        let player_life_state =
            memory_reader.read_u8(entity_pawn + client_dll::C_BaseEntity::m_lifeState);

        let player_active_weapon = memory_reader
            .read_usize(entity_pawn + client_dll::C_CSPlayerPawnBase::m_pClippingWeapon);

        let player_weapon_index = memory_reader.read_i16(
            player_active_weapon
                + client_dll::C_EconEntity::m_AttributeManager
                + client_dll::C_AttributeContainer::m_Item
                + client_dll::C_EconItemView::m_iItemDefinitionIndex,
        );

        let (tx, rx) = mpsc::channel();

        // Multi-thread id to string processing
        thread::spawn(move || {
            let weapon_enum = Weapon::from(player_weapon_index);
            let weapon_string = proccess_weapon(&weapon_enum);
            tx.send(weapon_string).unwrap();
        });

        let weapon_string = rx.recv().unwrap();

        let player_game_scene =
            memory_reader.read_usize(entity_pawn + client_dll::C_BaseEntity::m_pGameSceneNode);

        let player_bone_array = memory_reader
            .read_usize(player_game_scene + client_dll::CSkeletonInstance::m_modelState + 0x80);

        let mut player_bones = Vec::new();

        for bone in bone_connection {
            player_bones.push((
                memory_reader.read_vec_f32(player_bone_array + bone.bone1 * 32),
                memory_reader.read_vec_f32(player_bone_array + bone.bone2 * 32),
            ));
        }

        entities.push(Entity {
            name: player_name,
            pawn: entity_pawn,
            health: player_health,
            armor: player_armor,
            team: player_team,
            position: player_position,
            eye_position: player_eye_position,
            life_state: player_life_state,
            //active_weapon_index: player_weapon_index,
            active_weapon_name: weapon_string,
            bones: player_bones,
        });
    }

    // Return a vector with entities
    entities
}

// Function for reading local player
pub fn get_local_player(
    memory_reader: &MemoryReader,
    bone_connection: &[BoneConnection],
) -> Entity {
    let local_player_pawn =
        memory_reader.read_usize(memory_reader.module + client_dll_offsets::dwLocalPlayerPawn);

    let local_player_controller = memory_reader
        .read_usize(memory_reader.module + client_dll_offsets::dwLocalPlayerController);

    let player_name = memory_reader
        .read_string(local_player_controller + client_dll::CBasePlayerController::m_iszPlayerName);

    let player_health =
        memory_reader.read_i32(local_player_pawn + client_dll::C_BaseEntity::m_iHealth);

    let player_armor =
        memory_reader.read_i32(local_player_pawn + client_dll::C_CSPlayerPawn::m_ArmorValue);

    let player_team =
        memory_reader.read_i32(local_player_pawn + client_dll::C_BaseEntity::m_iTeamNum);

    let player_position =
        memory_reader.read_vec_f32(local_player_pawn + client_dll::C_BasePlayerPawn::m_vOldOrigin);

    let player_eye_position = player_position
        + memory_reader
            .read_vec_f32(local_player_pawn + client_dll::C_BaseModelEntity::m_vecViewOffset);

    let player_life_state =
        memory_reader.read_u8(local_player_pawn + client_dll::C_BaseEntity::m_lifeState);

    let player_active_weapon = memory_reader
        .read_usize(local_player_pawn + client_dll::C_CSPlayerPawnBase::m_pClippingWeapon);

    let player_weapon_index = memory_reader.read_i16(
        player_active_weapon
            + client_dll::C_EconEntity::m_AttributeManager
            + client_dll::C_AttributeContainer::m_Item
            + client_dll::C_EconItemView::m_iItemDefinitionIndex,
    );

    let (tx, rx) = mpsc::channel();

    // Multi-thread id to string processing
    thread::spawn(move || {
        let weapon_enum = Weapon::from(player_weapon_index);
        let weapon_string = proccess_weapon(&weapon_enum);
        tx.send(weapon_string).unwrap();
    });

    let weapon_string = rx.recv().unwrap();

    let player_game_scene =
        memory_reader.read_usize(local_player_pawn + client_dll::C_BaseEntity::m_pGameSceneNode);

    let player_bone_array = memory_reader
        .read_usize(player_game_scene + client_dll::CSkeletonInstance::m_modelState + 0x80);

    let mut player_bones = Vec::new();

    for bone in bone_connection {
        player_bones.push((
            memory_reader.read_vec_f32(player_bone_array + bone.bone1 * 32),
            memory_reader.read_vec_f32(player_bone_array + bone.bone2 * 32),
        ));
    }

    Entity {
        name: player_name,
        pawn: local_player_pawn,
        health: player_health,
        armor: player_armor,
        team: player_team,
        position: player_position,
        eye_position: player_eye_position,
        life_state: player_life_state,
        //active_weapon_index: player_weapon_index,
        active_weapon_name: weapon_string,
        bones: player_bones,
    }
}

// This function translates game player's position into 2d coordinates
// Stolen from https://www.unknowncheats.me/forum/4182733-post8.html
pub fn world_to_screen(
    position: &Vector3<f32>,
    matrix: &Matrix4<f32>,
    win_size: &[f32; 2],
) -> Vector2<f32> {
    let sight_x: f32 = win_size[0] / 2.0;
    let sight_y: f32 = win_size[1] / 2.0;

    let view = matrix[3][0] * position.x
        + matrix[3][1] * position.y
        + matrix[3][2] * position.z
        + matrix[3][3];

    if view <= 0.001 {
        return Vector2::new(0.0, 0.0);
    }

    let to_pos_x = sight_x
        + (matrix[0][0] * position.x
            + matrix[0][1] * position.y
            + matrix[0][2] * position.z
            + matrix[0][3])
            / view
            * sight_x;
    let to_pos_y = sight_y
        - (matrix[1][0] * position.x
            + matrix[1][1] * position.y
            + matrix[1][2] * position.z
            + matrix[1][3])
            / view
            * sight_y;

    Vector2::new(to_pos_x, to_pos_y)
}
