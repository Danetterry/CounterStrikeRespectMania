use egui::Color32;

// Just a structure with all parameters inside cheat
pub struct CheatOptions {
    pub line: ESPOption,
    pub esp_box: ESPOption,
    pub health_bar: HealthBarOption,
    pub bones: ESPOption,
    pub text: ESPOption,
}

pub struct ESPOption {
    pub enabled: bool,
    pub team_color: Color32,
    pub enemy_color: Color32,
}
pub struct HealthBarOption {
    pub enabled: bool,
    pub team_enabled: bool,
}

impl Default for CheatOptions {
    fn default() -> CheatOptions {
        CheatOptions {
            line: ESPOption { enabled: false, team_color: Color32::TRANSPARENT, enemy_color: Color32::GREEN },
            esp_box: ESPOption { enabled: true, team_color: Color32::TRANSPARENT, enemy_color: Color32::GREEN },
            health_bar: HealthBarOption { enabled: true, team_enabled: false, },
            bones: ESPOption { enabled: true, team_color: Color32::TRANSPARENT, enemy_color: Color32::GREEN },
            text: ESPOption { enabled: true, team_color: Color32::TRANSPARENT, enemy_color: Color32::WHITE },
        }
    }
}
