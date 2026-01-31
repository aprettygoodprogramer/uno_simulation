mod structs;
use structs::Game;

fn main() {
    let mut game_instance = Game::new();
    game_instance.add_cards_to_deck();
    game_instance.deal_hands(4);
    game_instance.start_game();
}
