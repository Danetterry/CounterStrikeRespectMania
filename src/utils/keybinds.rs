use winapi::ctypes::c_int;
use crate::utils::options::CheatOptions;

pub fn check_if_key_pressed(options: &mut CheatOptions, key_code: c_int) -> bool {
    let mut pressed: bool = false;
    
    unsafe {
        if (winapi::um::winuser::GetAsyncKeyState(key_code) != 0) && (!options.keybinds.is_key_pressed) {
            options.keybinds.is_key_pressed = true;
            pressed = true;
        } else if winapi::um::winuser::GetAsyncKeyState(key_code) == 0 {
            options.keybinds.is_key_pressed = false;
            pressed = false;
        }
    }
    
    pressed
}