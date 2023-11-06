use crate::world::entities::*;
use crate::world::terrain::*;
use crate::world::World;
use rand::seq::SliceRandom;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use termion::terminal_size;
use termion::{clear, cursor, event::Key, input::TermRead};

pub fn start<W: Write>(stdout: &mut W) -> Result<(), io::Error> {
    let mut world = World::new();
    let stdin = io::stdin();
    let mut keys = stdin.keys();

    'game_loop: loop {
        // render world
        let status_message = "You are in a forest. Watch out for the trees!";
        render(&world, stdout, status_message)?;

        // Read user input after rendering the world
        if let Some(key_event) = keys.next() {
            let key_event = key_event?;
            match key_event {
                Key::Up | Key::Down | Key::Left | Key::Right => move_player(&mut world, key_event),
                Key::Esc => break 'game_loop, // Exit game loop
                _ => {}
            }
        }

        move_npcs(&mut world);
        thread::sleep(Duration::from_millis(10));
    }

    Ok(())
}

fn move_player(world: &mut World, key_event: Key) {
    let (dx, dy) = match key_event {
        Key::Up => (0, -1),
        Key::Down => (0, 1),
        Key::Left => (-1, 0),
        Key::Right => (1, 0),
        _ => (0, 0),
    };
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

fn render<W: Write>(world: &World, stdout: &mut W, status_message: &str) -> Result<(), io::Error> {
    // Get the terminal size
    let (_, term_height) = terminal_size()?;

    // Calculate the positions for documentation and status message
    let _ = term_height - 1; // Assuming the status bar is at the bottom

    // draw the documentation on top
    let documentation = "This area displays helpful\r\ninformation about the game.";

    // Prepare full frame in a buffer
    let mut frame = format!("{}\r\n\r\n", documentation);

    // Draw the world into the buffer
    frame.push_str(&world.draw_to_string());

    // Build player inventory str
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
    // draw player inventory into the buffer
    frame.push_str(&format!("\r\n{}\r\n", inventory));

    // Draw the status message at the bottom into the buffer
    frame.push_str(&format!("\r\n{}\r\n", status_message));

    // Clear the screen and reset cursor position
    write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1))?;
    print!("{}", frame);

    // Flush stdout to ensure that all terminal output is displayed
    stdout.flush()?;

    Ok(())
}
