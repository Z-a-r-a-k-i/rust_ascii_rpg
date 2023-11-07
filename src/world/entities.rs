use crate::world::terrain::TileType;

pub struct Player {
    pub x: i32,
    pub y: i32,
    pub name: String,
    pub inventory: Vec<ItemType>,
    pub dead: bool,
}

pub struct NPC {
    pub x: i32,
    pub y: i32,
    pub npc_type: NPCType,
}

pub struct Chest {
    pub x: i32,
    pub y: i32,
}

#[derive(PartialEq, Eq)]
pub enum ItemType {
    Sword,
    Axe,
    Harpoon,
    Snorkel,
    Key,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum NPCType {
    Fish,
    Troll,
    Spider,
}

impl NPCType {
    // This function returns the TileType where the NPCType can move and spawn.
    pub fn allowed_tile(self) -> TileType {
        match self {
            NPCType::Fish => TileType::Water,
            NPCType::Troll => TileType::Grass,
            NPCType::Spider => TileType::Grass,
        }
    }
}
