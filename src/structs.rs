use rand::rng;
use rand::seq::SliceRandom;
use std::string;

#[derive(PartialEq, Eq)]
enum ReturnOption {
    PlusTwo,
    WildPlusFour,
    None,
    Skip,
    Reverse,
    Win,
}

#[derive(Clone)]
pub struct Card {
    color: String,
    kind: String,
    number: i32,
    plus: i8,
    wild: bool,
}

pub struct Player {
    hand: Vec<Card>,
}

impl Player {
    pub fn simulate_turn(&mut self, deck: &mut Deck) -> ReturnOption {
        let mut card_index_to_play: Option<usize> = None;

        for (i, card) in self.hand.iter().enumerate() {
            let color_match = card.color == deck.current_color;
            let number_match = card.number != -1 && card.number == deck.current_number;
            let symbol_match = card.kind != "Number" && card.kind == deck.current_kind;

            if color_match || number_match || symbol_match || card.wild {
                card_index_to_play = Some(i);
                break;
            }
        }

        match card_index_to_play {
            Some(index) => {
                let played_card = self.hand.remove(index);

                if played_card.wild {
                    deck.current_color = "Red".to_string();
                    deck.current_kind = played_card.kind.clone();
                    deck.current_number = -1;
                } else {
                    deck.current_color = played_card.color.clone();
                    deck.current_kind = played_card.kind.clone();
                    deck.current_number = played_card.number;
                }

                let result = if played_card.plus == 2 {
                    ReturnOption::PlusTwo
                } else if played_card.plus == 4 {
                    ReturnOption::WildPlusFour
                } else if played_card.kind == "Skip" {
                    ReturnOption::Skip
                } else if played_card.kind == "Reverse" {
                    ReturnOption::Reverse
                } else {
                    ReturnOption::None
                };

                deck.discard_pile.push(played_card);

                if self.hand.is_empty() {
                    return ReturnOption::Win;
                }

                result
            }
            None => {
                deck.reshuffle();
                if let Some(drawn_card) = deck.deck.pop() {
                    self.hand.push(drawn_card);
                }
                ReturnOption::None
            }
        }
    }
}

pub struct Game {
    pub players: Vec<Player>,
    pub deck: Deck,
    pub current_player: i32,
    pub direction: i8,
}

pub struct Deck {
    pub deck: Vec<Card>,
    pub discard_pile: Vec<Card>,
    current_color: String,
    current_number: i32,
    current_kind: String,
}

impl Deck {
    pub fn new() -> Self {
        Self {
            deck: vec![],
            discard_pile: vec![],
            current_color: String::new(),
            current_number: -1,
            current_kind: String::new(),
        }
    }
    fn reshuffle(&mut self) {
        if self.deck.is_empty() && self.discard_pile.len() > 1 {
            let top_card = self.discard_pile.pop().unwrap();

            self.deck.append(&mut self.discard_pile);
            self.shuffle_deck();

            self.discard_pile.push(top_card);
        }
    }
    fn add_color_cards(&mut self, color: &str) {
        self.deck.push(Card {
            color: color.to_string(),
            kind: "Number".to_string(),
            number: 0,
            plus: 0,
            wild: false,
        });

        for i in 1..=9 {
            for _ in 0..2 {
                self.deck.push(Card {
                    color: color.to_string(),
                    kind: "Number".to_string(),
                    number: i,
                    plus: 0,
                    wild: false,
                });
            }
        }

        for _ in 0..2 {
            self.deck.push(Card {
                color: color.to_string(),
                kind: "PlusTwo".to_string(),
                number: -1,
                plus: 2,
                wild: false,
            });
            self.deck.push(Card {
                color: color.to_string(),
                kind: "Reverse".to_string(),
                number: -1,
                plus: 0,
                wild: false,
            });
            self.deck.push(Card {
                color: color.to_string(),
                kind: "Skip".to_string(),
                number: -1,
                plus: 0,
                wild: false,
            });
        }
    }

    fn add_wild_cards(&mut self) {
        for _ in 0..4 {
            self.deck.push(Card {
                color: "Wild".to_string(),
                kind: "Wild".to_string(),
                number: -1,
                plus: 0,
                wild: true,
            });
            self.deck.push(Card {
                color: "Wild".to_string(),
                kind: "WildPlusFour".to_string(),
                number: -1,
                plus: 4,
                wild: true,
            });
        }
    }

    pub fn add_cards_to_deck(&mut self) {
        self.add_color_cards("Red");
        self.add_color_cards("Yellow");
        self.add_color_cards("Blue");
        self.add_color_cards("Green");
        self.add_wild_cards();
        self.shuffle_deck();
    }

    fn shuffle_deck(&mut self) {
        let mut rng = rng();
        self.deck.shuffle(&mut rng);
    }

    fn deal_8_cards(&mut self) -> Vec<Card> {
        let mut hand: Vec<Card> = vec![];
        for _ in 0..8 {
            if let Some(card) = self.deck.pop() {
                hand.push(card);
            }
        }
        hand
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            players: vec![],
            deck: Deck::new(),
            current_player: 0,
            direction: 1,
        }
    }
    pub fn add_cards_to_deck(&mut self) {
        self.deck.add_cards_to_deck();
    }
    pub fn deal_hands(&mut self, num_players: i32) {
        for _ in 0..num_players {
            let hand = self.deck.deal_8_cards();
            self.players.push(Player { hand });
        }
    }

    fn get_next_player_index(&self, steps: i32) -> i32 {
        let mut next = self.current_player + (self.direction as i32 * steps);
        let count = self.players.len() as i32;

        while next < 0 {
            next += count;
        }
        while next >= count {
            next -= count;
        }
        next
    }

    pub fn start_game(&mut self) {
        if let Some(starting_card) = self.deck.deck.pop() {
            self.deck.current_color = if starting_card.wild {
                "Red".to_string()
            } else {
                starting_card.color.clone()
            };
            self.deck.current_kind = starting_card.kind.clone();
            self.deck.current_number = starting_card.number;
            self.deck.discard_pile.push(starting_card);
        }

        loop {
            if self.current_player < 0 {
                self.current_player += self.players.len() as i32;
            }
            let player_index = (self.current_player as usize) % self.players.len();
            let player = &mut self.players[player_index];

            let returned_option = player.simulate_turn(&mut self.deck);

            if returned_option == ReturnOption::Win {
                return;
            }

            match returned_option {
                ReturnOption::Skip => {
                    self.current_player += (self.direction * 2) as i32;
                }
                ReturnOption::Reverse => {
                    self.direction *= -1;
                    self.current_player += self.direction as i32;
                }
                ReturnOption::PlusTwo => {
                    let victim_index = self.get_next_player_index(1) as usize;
                    for _ in 0..2 {
                        self.deck.reshuffle();
                        if let Some(c) = self.deck.deck.pop() {
                            self.players[victim_index].hand.push(c);
                        }
                    }
                    self.current_player += (self.direction * 2) as i32;
                }
                ReturnOption::WildPlusFour => {
                    let victim_index = self.get_next_player_index(1) as usize;
                    for _ in 0..4 {
                        self.deck.reshuffle();
                        if let Some(c) = self.deck.deck.pop() {
                            self.players[victim_index].hand.push(c);
                        }
                    }
                    self.current_player += (self.direction * 2) as i32;
                }
                _ => {
                    self.current_player += self.direction as i32;
                }
            }
        }
    }
    pub fn reset_game(&mut self) {
        self.players.clear();
        self.deck = Deck::new();
        self.current_player = 0;
        self.direction = 1;
    }
}
