mod structs;
use structs::Game;

fn main() {
    let mut amount_win = 0;
    let mut advance_win = 0;
    let mut basic_win = 0;
    loop {
        let mut game_instance = Game::new();

        game_instance.add_cards_to_deck();
        game_instance.deal_hands(2, 2);
        let winner = game_instance.start_game();
        game_instance.reset_game();
        amount_win += 1;
        match winner {
            structs::PlayerStrategy::Advanced => advance_win += 1,
            structs::PlayerStrategy::Basic => basic_win += 1,
        }
        if amount_win >= 1000000 {
            break;
        }
    }
    println!(
        "Total Games: {}, Advanced Wins: {}, Basic Wins: {}",
        amount_win, advance_win, basic_win
    );
}
