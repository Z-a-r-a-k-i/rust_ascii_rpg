use crate::world::entities::*;
use crate::world::terrain::*;
use crate::world::World;
use crossterm::{
    event::{self, Event as CEvent, KeyCode, KeyEvent},
    execute,
    terminal::{self, Clear, ClearType},
};
use rand::seq::SliceRandom;
use std::io::{self, Stdout, Write};
use std::time::Duration;

pub fn start(stdout: &mut Stdout) -> Result<(), io::Error> {
    // generate world
    let mut world = World::new();

    'game_loop: loop {
        // render world
        execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
        let status_message = "You are in a forest. Watch out for the trees!";
        render(&world, stdout, status_message)?;

        // Wait for user input
        let key_event = read_key_event()?;

        // Match the key code
        match key_event.code {
            KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => {
                move_player(&mut world, key_event)
            }
            KeyCode::Esc => break 'game_loop, // Exit game loop
            _ => {}
        }

        move_npcs(&mut world);
    }

    // If the loop exits normally, return Ok(())
    Ok(())
}

fn read_key_event() -> Result<KeyEvent, io::Error> {
    loop {
        if event::poll(Duration::from_millis(500))? {
            if let CEvent::Key(key_event) = event::read()? {
                return Ok(key_event);
            }
        }
    }
}

fn move_player(world: &mut World, key_event: KeyEvent) {
    let mut dx: i32 = 0;
    let mut dy: i32 = 0;
    match key_event.code {
        KeyCode::Up => {
            dx = 0;
            dy = -1;
        }
        KeyCode::Down => {
            dx = 0;
            dy = 1;
        }
        KeyCode::Left => {
            dx = -1;
            dy = 0;
        }
        KeyCode::Right => {
            dx = 1;
            dy = 0;
        }
        _ => {}
    }
    let destination_x = world.player.x + dx;
    let destination_y = world.player.y + dy;

    match world.terrain.tiles[destination_y as usize * TERRAIN_WIDTH + destination_x as usize] {
        TileType::Grass | TileType::Sand => {
            world.player.x = destination_x;
            world.player.y = destination_y;
        }
        _ => {}
    }
}

pub fn move_npcs(world: &mut World) {
    let mut rng = rand::thread_rng();

    'npc_loop: for npc in &mut world.npcs {
        // Generate a random number between 0 and 99 (inclusive)
        /*if rng.gen_range(0..100) < 20 {
            // 20% chance to not move
            return;
        }*/

        let mut directions = [(0, -1), (1, 0), (0, 1), (-1, 0)]; // Represents up, right, down, left
        directions.shuffle(&mut rng); // Shuffle the directions

        for &(dx, dy) in &directions {
            let new_x = npc.x + dx;
            let new_y = npc.y + dy;

            // Check if new position is player's position
            if new_x == world.player.x && new_y == world.player.y {
                continue; // Skip if new position is where the player is
            }

            // Check bounds and movable tile type
            if new_x >= 0
                && new_x < TERRAIN_WIDTH as i32
                && new_y >= 0
                && new_y < TERRAIN_HEIGHT as i32
            {
                let index = (new_y as usize) * TERRAIN_WIDTH + new_x as usize;
                if world.terrain.tiles[index] == npc.npc_type.allowed_tile() {
                    npc.x = new_x;
                    npc.y = new_y;
                    continue 'npc_loop; // Move successful, next npc
                }
            }
        }
        // If no valid move is found, the NPC stays in place.
    }
}

fn render(world: &World, stdout: &mut Stdout, status_message: &str) -> Result<(), io::Error> {
    // Get the terminal size
    let (_, term_height) = terminal::size()?;

    // Calculate the positions for documentation and status message
    let _ = term_height - 1; // Assuming the status bar is at the bottom

    // Clear the terminal
    execute!(stdout, Clear(ClearType::All))?;

    // draw the documentation on top
    let documentation = "This area displays helpful\r\ninformation about the game.";
    print!("{}\r\n\r\n", documentation);
    // Draw the world
    world.draw();

    // draw player inventory
    let mut inventory: String = String::from("\x1b[1m\x1b[93mPlayer Inventory:\x1b[0m ");
    for item in &world.player.inventory {
        match item {
            ItemType::Sword => {
                inventory.push_str("🗡️");
            }
            ItemType::Axe => {
                inventory.push_str("🪓");
            }
        }
    }
    print!("\r\n{}\r\n", inventory);

    // Draw the status message at the bottom
    print!("\r\n{}\r\n", status_message);

    // Flush stdout to ensure that all terminal output is displayed
    stdout.flush()?;

    Ok(())
}
