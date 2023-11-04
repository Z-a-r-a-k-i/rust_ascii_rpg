use crate::world::terrain::TileType;

pub struct Player {
    pub x: i32,
    pub y: i32,
    pub name: String,
    pub inventory: Vec<ItemType>,
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

pub enum ItemType {
    Sword,
    Axe,
}

#[derive(Copy, Clone)]
pub enum NPCType {
    Fish,
    Troll,
}

impl NPCType {
    // This function returns the TileType where the NPCType can move and spawn.
    pub fn allowed_tile(self) -> TileType {
        match self {
            NPCType::Fish => TileType::Water,
            NPCType::Troll => TileType::Grass,
        }
    }
}
