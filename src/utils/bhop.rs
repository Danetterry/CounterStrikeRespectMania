use std::thread;
use std::time::Duration;
use enigo::{Enigo, Key, Keyboard};
use enigo::Direction::{Press, Release};
use winapi;
use crate::offsets::client_dll::cs2_dumper::schemas::client_dll;
use crate::utils::entity::Entity;
use crate::utils::memory_reader::MemoryReader;
use crate::utils::options::CheatOptions;

pub fn perform_bunny_hop(local_player: &Entity, memory_reader: &MemoryReader, options: &mut CheatOptions, enigo: &mut Enigo) {
    if !options.bunny_hop.enabled {
        return;
    }

    thread::sleep(Duration::from_millis(20));

    let flag = memory_reader.read_u32(local_player.pawn + client_dll::C_BaseEntity::m_fFlags);

    if flag == 65664 {
        return;
    }
    
    enigo.key(Key::Space, Press).unwrap();
    thread::sleep(Duration::from_millis(20));
    enigo.key(Key::Space, Release).unwrap();  
}