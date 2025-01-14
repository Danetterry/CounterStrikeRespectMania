// Just a structure with all parameters inside cheat
pub struct CheatOptions {
    pub enable_line: bool,
    pub enable_box: bool,
    pub enable_text: bool,
}

impl Default for CheatOptions {
     fn default() -> CheatOptions {
        CheatOptions {
            enable_line: false,
            enable_box: true,
            enable_text: true,
        }
    }
}