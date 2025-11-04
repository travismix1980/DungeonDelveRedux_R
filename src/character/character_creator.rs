use super::class::PlayerClass;
use super::stats::Stats;
use super::gear::Item;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub class: PlayerClass,
    pub stats: Stats,
    pub gear: Vec<Item>,
    pub level: u8,
    pub xp: u32,
}

impl Player {
    pub fn new(class: PlayerClass) -> Self {
        match class {
            PlayerClass::Fighter => Self::create_fighter(),
            PlayerClass::Rogue => Self::create_rogue(),
            PlayerClass::Wizard => Self::create_wizard(),
            PlayerClass::Cleric => Self::create_cleric(),
        }
    }

    pub fn create_fighter() -> Self {
        Self {
            name: String::from("unamed"),
            class: PlayerClass::Fighter,
            stats: Stats {
                strength: 16,
                dexterity: 12,
                constitution: 15,
                intelligence: 8,
            },
            gear: vec![Item::Sword, Item::Shield, Item::Potion],
            level: 1,
            xp: 0,
        }
    }

    pub fn create_rogue() -> Self {
        Self {
            name: String::from("unamed"),
            class: PlayerClass::Rogue,
            stats: Stats {
                strength: 10,
                dexterity: 16,
                constitution: 12,
                intelligence: 10,
            },
            gear: vec![Item::Dagger, Item::Potion, Item::Torch],
            level: 1,
            xp: 0,
        }
    }

    pub fn create_wizard() -> Self {
        Self {
            name: String::from("unamed"),
            class: PlayerClass::Wizard,
            stats: Stats {
                strength: 6,
                dexterity: 10,
                constitution: 10,
                intelligence: 18,
            },
            gear: vec![Item::Staff, Item::Spellbook, Item::Potion],
            level: 1,
            xp: 0,
        }
    }

    pub fn create_cleric() -> Self {
        Self {
            name: String::from("unamed"),
            class: PlayerClass::Cleric,
            stats: Stats {
                strength: 12,
                dexterity: 10,
                constitution: 14,
                intelligence: 14,
            },
            gear: vec![Item::Mace, Item::HolySymbol, Item::Potion],
            level: 1,
            xp: 0,
        }
    }

    pub fn print_character(&self) {
        println!("Name: {}", self.name);
        println!("Class: {:?}", self.class);
        println!("Level: {} | XP: {}", self.level, self.xp);
        println!("Stats {:?}", self.stats);
        println!("Gear {:?}", self.gear);
    }
}
