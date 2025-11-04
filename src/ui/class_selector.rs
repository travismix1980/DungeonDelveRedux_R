use std::io;
use crate::character::class::PlayerClass;

// function to let the player chose their PlayerClass
pub fn select_class() -> PlayerClass {
    println!("Choose your class:");
    println!("1) Fighter");
    println!("2) Rogue");
    println!("3) Wizard");
    println!("4) Cleric");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read valid input");

    let class = match input.trim() {
        "1" => PlayerClass::Fighter,
        "2" => PlayerClass::Rogue,
        "3" => PlayerClass::Wizard,
        "4" => PlayerClass::Cleric,
        _ => {
            println!("Invalid Choice. Defaulting to Fighter.");
            PlayerClass::Fighter
        }
    };

    // return players chosen class
    class
}