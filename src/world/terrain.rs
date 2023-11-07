use rand::Rng;
pub const TERRAIN_WIDTH: usize = 100;
pub const TERRAIN_HEIGHT: usize = 50;
const FOREST_RADIUS: usize = 10;
const POND_RADIUS: usize = 15;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TileType {
    Grass,
    Tree,
    Water,
    Mountain,
    Sand,
    Castle,
    Chest,
    SpiderWeb,
    Heart,
}

pub struct Terrain {
    pub tiles: [TileType; TERRAIN_WIDTH * TERRAIN_HEIGHT],
    pub chest_found: bool,
}

impl Terrain {
    pub fn new() -> Self {
        // init terrain with grass tiles
        let mut terrain = Terrain {
            tiles: [TileType::Grass; TERRAIN_WIDTH * TERRAIN_HEIGHT],
            chest_found: false,
        };

        let mut rng = rand::thread_rng();

        // generate mountains around the border
        for y in 0..TERRAIN_HEIGHT {
            for x in 0..TERRAIN_WIDTH {
                if x == 0 || y == 0 || x == TERRAIN_WIDTH - 1 || y == TERRAIN_HEIGHT - 1 {
                    terrain.tiles[y * TERRAIN_WIDTH + x] = TileType::Mountain;
                }
            }
        }

        // generate diamond forest
        let center_forest_x = rng.gen_range(FOREST_RADIUS + 2..=TERRAIN_WIDTH - FOREST_RADIUS - 2);
        let center_forest_y = rng.gen_range(FOREST_RADIUS + 2..=TERRAIN_HEIGHT - FOREST_RADIUS - 2);

        for y in 0..TERRAIN_HEIGHT {
            for x in 0..TERRAIN_WIDTH {
                let dx = (center_forest_x as isize - x as isize).abs();
                let dy = (center_forest_y as isize - y as isize).abs();
                // Manhattan distance for a diamond shape
                if (dx + dy) as usize <= FOREST_RADIUS {
                    let index = y * TERRAIN_WIDTH + x;
                    terrain.tiles[index] = TileType::Tree;
                }
            }
        }

        // find a location for the pond that does not overlap with the forest
        let mut pond_center_x;
        let mut pond_center_y;
        let mut is_overlapping;

        loop {
            pond_center_x = rng.gen_range(POND_RADIUS + 4..=TERRAIN_WIDTH - POND_RADIUS - 4);
            pond_center_y = rng.gen_range(POND_RADIUS + 4..=TERRAIN_HEIGHT - POND_RADIUS - 4);

            // check for overlap
            is_overlapping = false;
            for y in (pond_center_y.saturating_sub(POND_RADIUS))
                ..=(pond_center_y + POND_RADIUS).min(TERRAIN_HEIGHT - 1)
            {
                for x in (pond_center_x.saturating_sub(POND_RADIUS))
                    ..=(pond_center_x + POND_RADIUS).min(TERRAIN_WIDTH - 1)
                {
                    let dx = pond_center_x as isize - x as isize;
                    let dy = pond_center_y as isize - y as isize;
                    if (dx * dx + dy * dy) as usize <= POND_RADIUS * POND_RADIUS {
                        // if within the pond radius, check if also within the forest
                        let forest_dx = (center_forest_x as isize - x as isize).abs();
                        let forest_dy = (center_forest_y as isize - y as isize).abs();
                        if (forest_dx + forest_dy) as usize <= FOREST_RADIUS {
                            is_overlapping = true;
                            break; // break inner loop
                        }
                    }
                }
                if is_overlapping {
                    break; // break outer loop if overlapping
                }
            }

            // if not overlapping, we found our pond location, break the loop
            if !is_overlapping {
                break;
            }
        }

        for y in 0..TERRAIN_HEIGHT {
            for x in 0..TERRAIN_WIDTH {
                let dx = pond_center_x as isize - x as isize;
                let dy = pond_center_y as isize - y as isize;
                // euclidean distance for a circle shape
                if (dx * dx + dy * dy) as usize <= POND_RADIUS * POND_RADIUS {
                    let index = y * TERRAIN_WIDTH + x;
                    // only replace the tile if it's currently grass to avoid overwriting trees
                    if let TileType::Grass = terrain.tiles[index] {
                        terrain.tiles[index] = TileType::Water;
                    }
                }
            }
        }

        // Add the castle tile in the middle of the pond
        let castle_index = pond_center_y * TERRAIN_WIDTH + pond_center_x;
        terrain.tiles[castle_index] = TileType::Castle;

        // Define a helper to check bounds and get tile indices
        let mut set_tile_if_in_bounds = |x, y, tile_type| {
            if x < TERRAIN_WIDTH && y < TERRAIN_HEIGHT {
                let index = y * TERRAIN_WIDTH + x;
                terrain.tiles[index] = tile_type;
            }
        };

        // Place grass around the castle
        for i in [-1, 0, 1] {
            for j in [-1, 0, 1] {
                if i != 0 || j != 0 {
                    // Don't replace the castle tile itself
                    set_tile_if_in_bounds(
                        pond_center_x.wrapping_add(i as usize),
                        pond_center_y.wrapping_add(j as usize),
                        TileType::Grass,
                    );
                }
            }
        }

        // Place a layer of sand around the grass
        for y in (pond_center_y.saturating_sub(POND_RADIUS + 2))
            ..=(pond_center_y + POND_RADIUS + 2).min(TERRAIN_HEIGHT - 1)
        {
            for x in (pond_center_x.saturating_sub(POND_RADIUS + 2))
                ..=(pond_center_x + POND_RADIUS + 2).min(TERRAIN_WIDTH - 1)
            {
                // We are iterating in a square around the castle
                // If it's not the center (castle) and not the immediate grass layer
                if !(x == pond_center_x && y == pond_center_y)
                    && !((x as isize - pond_center_x as isize).abs() <= 1
                        && (y as isize - pond_center_y as isize).abs() <= 1)
                {
                    // We check if it's adjacent to a grass tile that is adjacent to the castle
                    let mut adjacent_to_grass = false;
                    for i in -1..=1 {
                        for j in -1..=1 {
                            let adjacent_x = x.wrapping_add(i as usize);
                            let adjacent_y = y.wrapping_add(j as usize);
                            if adjacent_x < TERRAIN_WIDTH && adjacent_y < TERRAIN_HEIGHT {
                                let adjacent_index = adjacent_y * TERRAIN_WIDTH + adjacent_x;
                                if terrain.tiles[adjacent_index] == TileType::Grass
                                    && ((adjacent_x as isize - pond_center_x as isize).abs() <= 1
                                        && (adjacent_y as isize - pond_center_y as isize).abs()
                                            <= 1)
                                {
                                    adjacent_to_grass = true;
                                    break;
                                }
                            }
                        }
                        if adjacent_to_grass {
                            break;
                        }
                    }

                    if adjacent_to_grass {
                        let index = y * TERRAIN_WIDTH + x;
                        terrain.tiles[index] = TileType::Sand;
                    }
                }
            }
        }

        // Add a sand layer around the pond
        for y in (pond_center_y.saturating_sub(POND_RADIUS + 1))
            ..=(pond_center_y + POND_RADIUS + 1).min(TERRAIN_HEIGHT - 1)
        {
            for x in (pond_center_x.saturating_sub(POND_RADIUS + 1))
                ..=(pond_center_x + POND_RADIUS + 1).min(TERRAIN_WIDTH - 1)
            {
                let dx = pond_center_x as isize - x as isize;
                let dy = pond_center_y as isize - y as isize;
                let distance_squared = dx * dx + dy * dy;
                let index = y * TERRAIN_WIDTH + x;

                // Check for a ring around the pond to place sand
                if distance_squared as usize > POND_RADIUS * POND_RADIUS
                    && distance_squared as usize <= (POND_RADIUS + 1) * (POND_RADIUS + 1)
                    && (terrain.tiles[index] == TileType::Grass
                        || terrain.tiles[index] == TileType::Water)
                {
                    terrain.tiles[index] = TileType::Sand;
                }
            }
        }

        terrain
    }
}
