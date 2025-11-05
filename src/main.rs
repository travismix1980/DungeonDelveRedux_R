mod map_generation;
use crate::map_generation::room::{DungeonMap, RoomType};

struct Player {
    x: usize,
    y: usize,
}

fn describe_room(room: &RoomType, x: usize, y: usize) {
    println!("You are at ({}, {}).", x, y);
    println!("Room type: {:?}", room); // 🔍 Debug print
    match room {
        RoomType::Start => println!("The dungeon entrance looms before you."),
        RoomType::Empty => println!("Dusty stone walls. Nothing of interest."),
        RoomType::Wall => println!("A solid wall blocks your path."),
        RoomType::Monster => println!("You sense danger. A monster lurks nearby."),
        RoomType::Chest => println!("A chest sits in the corner. Could be loot."),
        RoomType::Boss => println!("A powerful presence fills the room… the boss awaits."),
        RoomType::Exit => println!("You found the exit!  The dungeon fades behind you..."),
    }
}

fn move_player(input: &str, player: &mut Player, map: &DungeonMap) {
    let (x, y) = (player.x, player.y);
    let (new_x, new_y) = match input.trim() {
        "w" if y > 0 => (x, y - 1),
        "s" if y + 1 < map.height() => (x, y + 1),
        "a" if x > 0 => (x - 1, y),
        "d" if x + 1 < map.width() => (x + 1, y),
        _ => {
            println!("Invalid move.");
            return;
        }
    };

    match map.get_room(new_x, new_y) {
        Some(RoomType::Wall) => println!("You bump into a wall."),
        Some(_) => {
            player.x = new_x;
            player.y = new_y;
            println!("Moved to ({}, {})", player.x, player.y); // 🔍 Debug print
        }
        None => println!("Out of bounds."),
    }
}

fn main() {
    let map = DungeonMap::new();
    let mut player = Player { x: 4, y: 4 };

    loop {
        if let Some(room) = map.get_room(player.x, player.y) {
            describe_room(room, player.x, player.y);

            // exit condition
            if let RoomType::Exit = room {
                break;
            }
        }

        println!("Move with WASD:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        move_player(&input, &mut player, &map);
    }

    println!("Game Over. Thanks for playing");
}