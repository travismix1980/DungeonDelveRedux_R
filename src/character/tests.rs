use super::character_creator::Player;
use super::class::PlayerClass;
use super::gear::Item;

// test our class creation for fighter
#[test]
fn test_create_fighter() {
    let player = Player::create_fighter();
    assert_eq!(player.class, PlayerClass::Fighter);
    assert!(player.gear.contains(&Item::Sword));
    assert_eq!(player.stats.strength, 16);
    assert_eq!(player.name.to_lowercase(), "unamed");
}

// test our class creation for rogue
#[test]
fn test_create_rogue() {
    let player = Player::create_rogue();
    assert_eq!(player.class, PlayerClass::Rogue);
    assert!(player.gear.contains(&Item::Dagger));
    assert_eq!(player.stats.dexterity, 16);
    assert_eq!(player.name.to_lowercase(), "unamed");
}

// test our class creation for wizard
#[test]
fn test_create_wizard() {
    let player = Player::create_wizard();
    assert_eq!(player.class, PlayerClass::Wizard);
    assert!(player.gear.contains(&Item::Spellbook));
    assert_eq!(player.stats.intelligence, 18);
    assert_eq!(player.name.to_lowercase(), "unamed");
}

// test our class creation for cleric
#[test]
fn test_create_cleric() {
    let player = Player::create_cleric();
    assert_eq!(player.class, PlayerClass::Cleric);
    assert!(player.gear.contains(&Item::Mace));
    assert_eq!(player.stats.constitution, 14);
    assert_eq!(player.name.to_lowercase(), "unamed");
}

// test our class creation pattern matching routing used in Player::new()
#[test]
fn test_player_new_routing() {
    let fighter = Player::new(PlayerClass::Fighter);
    let rogue = Player::new(PlayerClass::Rogue);
    let wizard = Player::new(PlayerClass::Wizard);
    let cleric = Player::new(PlayerClass::Cleric);

    assert_eq!(fighter.class, PlayerClass::Fighter);
    assert_eq!(rogue.class, PlayerClass::Rogue);
    assert_eq!(wizard.class, PlayerClass::Wizard);
    assert_eq!(cleric.class, PlayerClass::Cleric);
}