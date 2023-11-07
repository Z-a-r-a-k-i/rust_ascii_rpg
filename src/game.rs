use crate::world::entities::*;
use crate::world::terrain::*;
use crate::world::World;
use rand::seq::SliceRandom;
use rand::Rng;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use termion::terminal_size;
use termion::{clear, cursor, event::Key, input::TermRead};

pub fn start<W: Write>(stdout: &mut W) -> Result<(), io::Error> {
    let stdin = io::stdin();
    let mut keys = stdin.keys();

    let mut world = World::new();
    let mut status_message = String::from( "You are in a bizarre world full of weird emojis. And what am I doing inside a linux terminal?");
    render(&world, stdout, &status_message)?;

    'game_loop: loop {
        // Read user input after rendering the world
        if let Some(key_event) = keys.next() {
            let key_event = key_event?;
            match key_event {
                Key::Up | Key::Down | Key::Left | Key::Right => {
                    status_message = move_player(&mut world, key_event);
                }
                Key::Esc => break 'game_loop, // Exit game loop
                _ => {}
            }
        }

        move_npcs(&mut world);
        // render world
        render(&world, stdout, &status_message)?;

        // handle player death
        if world.player.dead {
            while let Some(key_event) = keys.next() {
                if let Ok(Key::Char('\n')) = key_event {
                    break 'game_loop;
                }
            }
            break 'game_loop;
        }

        thread::sleep(Duration::from_millis(10));
    }

    Ok(())
}

fn move_player(world: &mut World, key_event: Key) -> String {
    let (dx, dy) = match key_event {
        Key::Up => (0, -1),
        Key::Down => (0, 1),
        Key::Left => (-1, 0),
        Key::Right => (1, 0),
        _ => (0, 0),
    };
    let destination_x = world.player.x + dx;
    let destination_y = world.player.y + dy;

    if destination_x < 0
        || destination_y < 0
        || destination_x >= TERRAIN_WIDTH as i32
        || destination_y >= TERRAIN_HEIGHT as i32
    {
        return "Stay with us, don't try to leave".to_string(); // Prevent moving out of bounds
    }

    match world.terrain.tiles[destination_y as usize * TERRAIN_WIDTH + destination_x as usize] {
        TileType::Grass | TileType::Sand => {
            world.player.x = destination_x;
            world.player.y = destination_y;
            // check for troll to interact with it
            if let Some(npc_index) = world.npcs.iter().position(|npc| {
                npc.x == destination_x && npc.y == destination_y && npc.npc_type == NPCType::Troll
            }) {
                if world.player.inventory.contains(&ItemType::Sword) {
                    world.npcs.remove(npc_index); // Remove the troll NPC from the game
                    if !world.player.inventory.contains(&ItemType::Axe) {
                        world.player.inventory.push(ItemType::Axe);
                        return "You bravely fight and defeat the troll! He leaves an Axe on the ground!".to_string();
                    } else {
                        return "You bravely fight and defeat the troll!".to_string();
                    }
                } else {
                    return "There's a troll here! You need a sword to fight!".to_string();
                }
            }
            // Check for spider to interact with it
            if let Some(npc_index) = world.npcs.iter().position(|npc| {
                npc.x == destination_x && npc.y == destination_y && npc.npc_type == NPCType::Spider
            }) {
                world.npcs.remove(npc_index); // Remove the spider NPC from the game
                world.player.inventory.push(ItemType::Snorkel); // Loot a snorkel
                return "You've defeated the spider and found a snorkel on its body!".to_string();
            }
            return "You are wandering around!".to_string();
        }
        // player can move on water only if he has the snorkel in his inventory
        TileType::Water => {
            // check for fish to interact with it
            if let Some(npc_index) = world.npcs.iter().position(|npc| {
                npc.x == destination_x && npc.y == destination_y && npc.npc_type == NPCType::Fish
            }) {
                if world.player.inventory.contains(&ItemType::Harpoon) {
                    if world.player.inventory.contains(&ItemType::Key) {
                        return "You don't want to fish anymore!".to_string();
                    } else {
                        world.npcs.remove(npc_index); // Remove the fish NPC from the game
                        world.player.inventory.push(ItemType::Key);
                        return "You catch a fish with your harpoon! When you look inside the fish, you find a key.. Looks like the fish had something weird for dinner!".to_string();
                    }
                } else {
                    return "You see a fish swimming by, but you have no tool to catch it!"
                        .to_string();
                }
            } else {
                if world.player.inventory.contains(&ItemType::Snorkel) {
                    world.player.x = destination_x;
                    world.player.y = destination_y;
                    return "You are swimming like a cute little fish !".to_string();
                } else {
                    return "You cannot enter like this in the water, look around for something that may help you!".to_string();
                }
            }
        }
        TileType::Tree => {
            if world.player.inventory.contains(&ItemType::Axe) {
                let mut rng = rand::thread_rng();
                let destination_idx =
                    destination_y as usize * TERRAIN_WIDTH + destination_x as usize;
                if rng.gen_range(0..100) < 15 && !world.terrain.chest_found {
                    // 15% chance to replace the tree with a chest
                    world.terrain.tiles[destination_idx] = TileType::Chest;
                    world.terrain.chest_found = true;
                    return "You chopped a tree... And found a chest!".to_string();
                } else {
                    // Otherwise, replace it with grass
                    world.terrain.tiles[destination_idx] = TileType::Grass;
                    return "You chopped a tree!".to_string();
                }
            }
            return "You gonna need a tool if you want to interact with a tree!".to_string();
        }
        TileType::Chest => {
            if world.player.inventory.contains(&ItemType::Harpoon) {
                return "Chest is empty. You already took the harpoon that was inside.".to_string();
            } else {
                world.player.inventory.push(ItemType::Harpoon);
                return "You found a harpoon, but for what ?".to_string();
            }
        }
        TileType::Castle => {
            if world.player.inventory.contains(&ItemType::Key) {
                world.terrain.tiles
                    [destination_y as usize * TERRAIN_WIDTH + destination_x as usize] =
                    TileType::Heart;
                return "You enter the castle and there is a beautiful princess inside, you found love and are virtually happy for the rest of your virtual life. You can exit the simulation! Press Escape..".to_string();
            }
            return "Castle door is locked!".to_string();
        }
        TileType::SpiderWeb => {
            world.player.dead = true;
            return "It's a trap!!! You are trapped into the spider web, the spider is gonna come back soon and eat you alive. You die. Press Enter to continue...".to_string();
        }
        _ => {
            return "THIS MESSAGE SHOULD NEVER APPEAR, CONTACT THE GAME DEVELOPER!".to_string();
        }
    }
}

pub fn move_npcs(world: &mut World) {
    let mut rng = rand::thread_rng();

    'npc_loop: for npc in &mut world.npcs {
        // Generate a random number between 0 and 99 (inclusive)
        if rng.gen_range(0..100) < 10 {
            // 10% chance to not move
            continue;
        }
        // Check if the NPC is a spider and generate a random number
        if npc.npc_type == NPCType::Spider && rng.gen_range(0..100) < 15 {
            // 15% chance to change the current tile to SpiderWeb
            let index = (npc.y as usize) * TERRAIN_WIDTH + npc.x as usize;
            world.terrain.tiles[index] = TileType::SpiderWeb;
        }

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
                inventory.push_str("🗡️  ");
            }
            ItemType::Axe => {
                inventory.push_str("🪓  ");
            }
            ItemType::Snorkel => {
                inventory.push_str("🤿  ");
            }
            ItemType::Harpoon => {
                inventory.push_str("🔱  ");
            }
            ItemType::Key => {
                inventory.push_str("🗝️  ");
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
