use rand::Rng;
pub mod entities;
pub mod terrain;
pub use entities::*;
pub use terrain::Terrain;

pub struct World {
    pub terrain: Terrain,
    pub player: Player,
    pub npcs: Vec<NPC>,
}

impl World {
    pub fn new() -> World {
        let new_terrain = Terrain::new();

        // Spawn player in grass
        let (player_x, player_y) =
            World::find_spawn_location(&new_terrain, terrain::TileType::Grass);

        let mut world = Self {
            terrain: new_terrain,
            player: Player {
                x: player_x,
                y: player_y,
                name: "alk".to_string(),
                inventory: vec![ItemType::Sword],
            },
            npcs: Vec::new(),
        };

        // spawn fishes
        for _ in 0..3 {
            world.spawn_npc(NPCType::Fish);
        }

        // spawn trolls
        for _ in 0..5 {
            world.spawn_npc(NPCType::Troll);
        }

        return world;
    }
    fn find_spawn_location(terrain: &Terrain, tile_type: terrain::TileType) -> (i32, i32) {
        let mut rng = rand::thread_rng();

        loop {
            let x = rng.gen_range(0..terrain::TERRAIN_WIDTH);
            let y = rng.gen_range(0..terrain::TERRAIN_HEIGHT);
            let index = y * terrain::TERRAIN_WIDTH + x;

            if terrain.tiles[index] == tile_type {
                return (x as i32, y as i32);
            }
        }
    }
    fn spawn_npc(&mut self, npc_type: NPCType) {
        // find spawn location
        let (x, y) = World::find_spawn_location(&self.terrain, npc_type.allowed_tile());
        self.npcs.push(NPC {
            x: x as i32,
            y: y as i32,
            npc_type: npc_type,
        })
    }
    pub fn draw(&self) {
        for y in 0..terrain::TERRAIN_HEIGHT {
            for x in 0..terrain::TERRAIN_WIDTH {
                if x == self.player.x as usize && y == self.player.y as usize {
                    print!("🏃");
                } else {
                    // Check if any NPC is at the current position
                    let mut npc_drawn = false;
                    for npc in &self.npcs {
                        if x == npc.x as usize && y == npc.y as usize {
                            // Based on the NPC type, print the corresponding symbol
                            let npc_symbol = match npc.npc_type {
                                NPCType::Fish => "🐠",
                                NPCType::Troll => "👹",
                                // Add other NPC types here as needed
                            };
                            print!("{}", npc_symbol);
                            npc_drawn = true;
                            break; // No need to check other NPCs since one is already found at this position
                        }
                    }
                    // If no NPC was drawn, draw the terrain
                    if !npc_drawn {
                        let index = y * terrain::TERRAIN_WIDTH + x;
                        let symbol = match self.terrain.tiles[index] {
                            terrain::TileType::Grass => '🟩',
                            terrain::TileType::Tree => '🌲',
                            terrain::TileType::Water => '💧',
                            terrain::TileType::Mountain => '🗻',
                            terrain::TileType::Sand => '🟨',
                            terrain::TileType::Castle => '🏰',
                        };
                        print!("{}", symbol);
                    }
                }
            }
            print!("\r\n"); // Newline at the end of each row
        }
    }

    pub fn draw_to_string(&self) -> String {
        let mut buffer = String::new();

        for y in 0..terrain::TERRAIN_HEIGHT {
            for x in 0..terrain::TERRAIN_WIDTH {
                if x == self.player.x as usize && y == self.player.y as usize {
                    buffer.push_str("🏃");
                } else {
                    let mut npc_drawn = false;
                    for npc in &self.npcs {
                        if x == npc.x as usize && y == npc.y as usize {
                            let npc_symbol = match npc.npc_type {
                                NPCType::Fish => "🐠",
                                NPCType::Troll => "👹",
                                // Add other NPC types here as needed
                            };
                            buffer.push_str(npc_symbol);
                            npc_drawn = true;
                            break; // Only one NPC can occupy a tile, no need to check others
                        }
                    }

                    if !npc_drawn {
                        let index = y * terrain::TERRAIN_WIDTH + x;
                        let symbol = match self.terrain.tiles[index] {
                            terrain::TileType::Grass => '🟩',
                            terrain::TileType::Tree => '🌲',
                            terrain::TileType::Water => '💧',
                            terrain::TileType::Mountain => '🗻',
                            terrain::TileType::Sand => '🟨',
                            terrain::TileType::Castle => '🏰',
                            // ... add other tile types here as needed
                        };
                        buffer.push(symbol);
                    }
                }
            }
            buffer.push_str("\r\n"); // Newline at the end of each row
        }

        buffer
    }
}
