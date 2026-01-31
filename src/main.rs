mod structs;
use structs::{Card, Game, Player};
fn main() {
    let mut game_instance = Game {
        players: vec![],
        deck: vec![],
        discard_pile: vec![],
        current_player: 0,
        direction: 1,
    };
    game_instance.add_cards_to_deck();

    println!("Deck has {} cards", game_instance.deck.len());
}
