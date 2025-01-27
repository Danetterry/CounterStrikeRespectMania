use enigo::{Enigo, Key, Keyboard};
use enigo::Direction::{Press, Release};
use winapi::um::winuser::VK_SPACE;
use crate::offsets::client_dll::cs2_dumper::schemas::client_dll;
use crate::utils::entity::Entity;
use crate::utils::memory_reader::MemoryReader;
use crate::utils::options::CheatOptions;

pub fn perform_bunny_hop(local_player: &Entity, memory_reader: &MemoryReader, options: &mut CheatOptions, enigo: &mut Enigo) {
    if !options.bunny_hop.enabled {
        return;
    }

    let flag = memory_reader.read_u32(local_player.pawn + client_dll::C_BaseEntity::m_fFlags);           

    if flag == 65664 {
        return;
    }
    
    unsafe {
        if winapi::um::winuser::GetAsyncKeyState(VK_SPACE) != 0 {
            enigo.key(Key::F24, Press).unwrap();
            enigo.key(Key::F24, Release).unwrap();
        }
    }
}