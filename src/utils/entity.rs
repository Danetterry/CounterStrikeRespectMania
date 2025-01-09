use crate::offsets::offsets::cs2_dumper::offsets::client_dll as client_dll_offsets;
use crate::offsets::client_dll::cs2_dumper::schemas::client_dll;
use crate::utils::utils::MemoryReader;

// This structure contains all needed thing about player
pub struct Entity {
    pub name: String,
    pub health: i32,
    pub team: i32
}

pub fn get_all_entities(memory_reader: &MemoryReader) -> Vec<Entity> {
    // Creating vector with entities
    let mut entities: Vec<Entity> = Vec::new();

    // This loop was stolen from https://www.unknowncheats.me/forum/3907391-post4.html
    for i in 0..64 {
        let entity = memory_reader.read_usize(memory_reader.module + client_dll_offsets::dwEntityList);

        let mut list_entity = memory_reader.read_usize(entity + ((8 * (i & 0x7FFF) >> 9) + 16));
        if list_entity == 0 {
            continue;
        }

        let entity_controller = memory_reader.read_usize(list_entity + (120) * (i & 0x1FF));
        if entity_controller == 0 {
            continue;
        }

        let entity_controller_pawn = memory_reader.read_usize(entity_controller + client_dll::CCSPlayerController::m_hPlayerPawn);
        if entity_controller_pawn == 0 {
            continue;
        }

        list_entity = memory_reader.read_usize(entity + (0x8 * ((entity_controller_pawn & 0x7FFF) >> 9) + 16));

        let entity_pawn = memory_reader.read_usize(list_entity + (120) * (entity_controller_pawn & 0x1FF));

        let player_team = memory_reader.read_i32(entity_pawn + client_dll::C_BaseEntity::m_iTeamNum);
        let player_health = memory_reader.read_i32(entity_pawn + client_dll::C_BaseEntity::m_iHealth);

        let player_name = memory_reader.read_string(entity_controller + client_dll::CBasePlayerController::m_iszPlayerName, 8);

        entities.push(Entity {name: player_name, health: player_health, team: player_team});
    }

    // Return a vector with entities
    entities
}