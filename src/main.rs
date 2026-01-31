mod structs;
use structs::Game;

fn main() {
    let mut amount_win = 0;

    loop {
        let mut game_instance = Game::new();

        game_instance.add_cards_to_deck();
        game_instance.deal_hands(100);
        game_instance.start_game();
        game_instance.reset_game();
        amount_win += 1;
        println!("Games won: {}", amount_win);
    }
}
