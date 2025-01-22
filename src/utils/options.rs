use egui::Color32;

// Just a structure with all parameters inside cheat
pub struct CheatOptions {
    pub line: ESPOption,
    pub esp_box: ESPOption,
    pub health_bar: BarOption,
    pub armor_bar: BarOption,
    pub bones: ESPOption,
    pub text: ESPOption,
    pub bunny_hop: BhopOptions,
    pub bomb_timer: SubwindowOptions,
    pub bomb: BombESPOption,
}

pub struct ESPOption {
    pub enabled: bool,
    pub team_color: Color32,
    pub enemy_color: Color32,
}

pub struct BombESPOption {
    pub enabled: bool,
    pub color: Color32,
}

pub struct BarOption {
    pub enabled: bool,
    pub team_enabled: bool,
}

pub struct BhopOptions {
    pub enabled: bool,
    pub flag: bool,
    pub in_jump: bool,
}

pub struct SubwindowOptions {
    pub enabled: bool,
    pub resizable: bool,
    pub y_offset: f32,
}

impl Default for CheatOptions {
    fn default() -> CheatOptions {
        CheatOptions {
            line: ESPOption { enabled: false, team_color: Color32::TRANSPARENT, enemy_color: Color32::GREEN },
            esp_box: ESPOption { enabled: true, team_color: Color32::TRANSPARENT, enemy_color: Color32::GREEN },
            health_bar: BarOption { enabled: true, team_enabled: false },
            armor_bar: BarOption { enabled: true, team_enabled: false },
            bones: ESPOption { enabled: true, team_color: Color32::TRANSPARENT, enemy_color: Color32::GREEN },
            text: ESPOption { enabled: true, team_color: Color32::TRANSPARENT, enemy_color: Color32::WHITE },
            bunny_hop: BhopOptions { enabled: false, flag: false, in_jump: false },
            bomb_timer: SubwindowOptions { enabled: true, resizable: true, y_offset: 100.0, },
            bomb: BombESPOption { enabled: true, color: Color32::WHITE }
        }
    }
}
