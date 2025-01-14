use std::convert::From;

// Weapons
#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(i16)]
pub enum Weapon {
    Deagle = 1,
    Elite = 2,
    FiveSeven = 3,
    Glock = 4,
    Ak47 = 7,
    Aug = 8,
    Awp = 9,
    Famas = 10,
    G3SG1 = 11,
    Galil = 13,
    M249 = 14,
    M4A4 = 16,
    Mac10 = 17,
    P90 = 19,
    MP5SD = 23,
    Ump45 = 24,
    Xm1014 = 25,
    Bizon = 26,
    Mag7 = 27,
    Negev = 28,
    Sawedoff = 29,
    Tec9 = 30,
    Zeus = 31,
    P2000 = 32,
    Mp7 = 33,
    Mp9 = 34,
    Nova = 35,
    P250 = 36,
    Scar20 = 38,
    Sg556 = 39,
    Ssg08 = 40,
    CtKnife = 42,
    Flashbang = 43,
    HE = 44,
    Smoke = 45,
    Molotov = 46,
    Decoy = 47,
    Incgrenade = 48,
    C4 = 49,
    Butterfly = 51,
    Healthshot = 57,
    TKnife = 59,
    M4A1S = 60,
    UspS = 61,
    Cz75A = 63,
    Revolver = 64,
    Bayonet = 500,
    Flip = 505,
    Gut = 506,
    Karambit = 507,
    M9 = 508,
    Tactical = 509,
    Falchion = 512,
    Bowie = 514,
    Stilleto = 522,
    Skeleton = 525,
}

// Transform id into enum function
impl From<i16> for Weapon {
    fn from(value: i16) -> Self {
        match value {
            1 => Weapon::Deagle,
            2 => Weapon::Elite,
            3 => Weapon::FiveSeven,
            4 => Weapon::Glock,
            7 => Weapon::Ak47,
            8 => Weapon::Aug,
            9 => Weapon::Awp,
            10 => Weapon::Famas,
            11 => Weapon::G3SG1,
            13 => Weapon::Galil,
            14 => Weapon::M249,
            16 => Weapon::M4A4,
            17 => Weapon::Mac10,
            19 => Weapon::P90,
            23 => Weapon::MP5SD,
            24 => Weapon::Ump45,
            25 => Weapon::Xm1014,
            26 => Weapon::Bizon,
            27 => Weapon::Mag7,
            28 => Weapon::Negev,
            29 => Weapon::Sawedoff,
            30 => Weapon::Tec9,
            31 => Weapon::Zeus,
            32 => Weapon::P2000,
            33 => Weapon::Mp7,
            34 => Weapon::Mp9,
            35 => Weapon::Nova,
            36 => Weapon::P250,
            38 => Weapon::Scar20,
            39 => Weapon::Sg556,
            40 => Weapon::Ssg08,
            42 => Weapon::CtKnife,
            43 => Weapon::Flashbang,
            44 => Weapon::HE,
            45 => Weapon::Smoke,
            46 => Weapon::Molotov,
            47 => Weapon::Decoy,
            48 => Weapon::Incgrenade,
            49 => Weapon::C4,
            51 => Weapon::Butterfly,
            57 => Weapon::Healthshot,
            59 => Weapon::TKnife,
            60 => Weapon::M4A1S,
            61 => Weapon::UspS,
            63 => Weapon::Cz75A,
            64 => Weapon::Revolver,
            500 => Weapon::Bayonet,
            505 => Weapon::Flip,
            506 => Weapon::Gut,
            507 => Weapon::Karambit,
            508 => Weapon::M9,
            509 => Weapon::Tactical,
            512 => Weapon::Falchion,
            514 => Weapon::Bowie,
            522 => Weapon::Stilleto,
            525 => Weapon::Skeleton,
            _ => Weapon::Revolver,
        }
    }
}

// Function to proccess weapon into string
pub fn proccess_weapon(weapon: &Weapon) -> String {
    match weapon {
        Weapon::Deagle => String::from("Deagle"),
        Weapon::Elite => String::from("Elite"),
        Weapon::FiveSeven => String::from("FiveSeven"),
        Weapon::Glock => String::from("Glock"),
        Weapon::Ak47 => String::from("AK-47"),
        Weapon::Aug => String::from("AUG"),
        Weapon::Awp => String::from("AWP"),
        Weapon::Famas => String::from("Famas"),
        Weapon::G3SG1 => String::from("G3SG1"),
        Weapon::Galil => String::from("Galil"),
        Weapon::M249 => String::from("M249"),
        Weapon::M4A4 => String::from("M4A4"),
        Weapon::Mac10 => String::from("Mac10"),
        Weapon::P90 => String::from("P90"),
        Weapon::MP5SD => String::from("MP5SD"),
        Weapon::Ump45 => String::from("UMP-45"),
        Weapon::Xm1014 => String::from("XM1014"),
        Weapon::Bizon => String::from("Bizon"),
        Weapon::Mag7 => String::from("Mag7"),
        Weapon::Negev => String::from("Negev"),
        Weapon::Sawedoff => String::from("Sawedoff"),
        Weapon::Tec9 => String::from("Tec-9"),
        Weapon::Zeus => String::from("Zeus"),
        Weapon::P2000 => String::from("P2000"),
        Weapon::Mp7 => String::from("MP7"),
        Weapon::Mp9 => String::from("MP9"),
        Weapon::Nova => String::from("Nova"),
        Weapon::P250 => String::from("P250"),
        Weapon::Scar20 => String::from("Scar-20"),
        Weapon::Sg556 => String::from("SG 556"),
        Weapon::Ssg08 => String::from("SSG 08"),
        Weapon::CtKnife => String::from("Knife (CT)"),
        Weapon::Flashbang => String::from("Flashbang"),
        Weapon::HE => String::from("HE Grenade"),
        Weapon::Smoke => String::from("Smoke Grenade"),
        Weapon::Molotov => String::from("Molotov"),
        Weapon::Decoy => String::from("Decoy Grenade"),
        Weapon::Incgrenade => String::from("Incendiary Grenade"),
        Weapon::C4 => String::from("C4"),
        Weapon::Butterfly => String::from("Butterfly Knife"),
        Weapon::Healthshot => String::from("Healthshot"),
        Weapon::TKnife => String::from("Knife (T)"),
        Weapon::M4A1S => String::from("M4A1-S"),
        Weapon::UspS => String::from("USP-S"),
        Weapon::Cz75A => String::from("CZ75-Auto"),
        Weapon::Revolver => String::from("R8 Revolver"),
        Weapon::Bayonet => String::from("Bayonet"),
        Weapon::Flip => String::from("Flip Knife"),
        Weapon::Gut => String::from("Gut Knife"),
        Weapon::Karambit => String::from("Karambit"),
        Weapon::M9 => String::from("M9 Bayonet"),
        Weapon::Tactical => String::from("Huntsman Knife"), // Assuming Tactical refers to Huntsman Knife
        Weapon::Falchion => String::from("Falchion Knife"),
        Weapon::Bowie => String::from("Bowie Knife"),
        Weapon::Stilleto => String::from("Stiletto Knife"),
        Weapon::Skeleton => String::from("Skeleton Knife"),
    }
}