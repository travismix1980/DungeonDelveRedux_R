mod character;
mod ui;

use character::character_creator::Player;
use ui::class_selector::select_class;


fn main() {
    let class = select_class();
    let player = Player::new(class);
    player.print_character();
}
