use rand::rng;
use rand::seq::SliceRandom;

pub struct Card {
    color: String,
    number: i32,
    plus: i8,
    wild: bool,
}

pub struct Player {
    hand: Vec<Card>,
}

pub struct Game {
    pub players: Vec<Player>,
    pub deck: Vec<Card>,
    pub discard_pile: Vec<Card>,
    pub current_player: i32,
    pub direction: i8,
}

struct Deck {
    deck: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        Self { deck: vec![] }
    }
    fn add_color_cards(&mut self, color: &str) {
        let mut amount_cards = 0;
        while amount_cards != 10 {
            if amount_cards == 0 {
                self.deck.push(Card {
                    color: color.to_string(),
                    number: 0,
                    plus: 0,
                    wild: false,
                });
            } else {
                self.deck.push(Card {
                    color: color.to_string(),
                    number: (amount_cards),
                    plus: 0,
                    wild: false,
                });
                self.deck.push(Card {
                    color: color.to_string(),
                    number: (amount_cards),
                    plus: 0,
                    wild: false,
                });
            }
            amount_cards += 1;
        }

        amount_cards = 0;
        while amount_cards != 2 {
            self.deck.push(Card {
                color: "P2".to_string(),
                number: 0,
                plus: 2,
                wild: false,
            });
            self.deck.push(Card {
                color: "Reverse".to_string(),
                number: 0,
                plus: 0,
                wild: false,
            });
            self.deck.push(Card {
                color: "Skip".to_string(),
                number: 0,
                plus: 0,
                wild: false,
            });
            amount_cards += 1;
        }
    }
    fn add_wild_cards(&mut self) {
        let mut amount_cards = 0;
        while amount_cards != 4 {
            self.deck.push(Card {
                color: "Wild".to_string(),
                number: 0,
                plus: 0,
                wild: true,
            });
            self.deck.push(Card {
                color: "Wild+4".to_string(),
                number: 0,
                plus: 4,
                wild: true,
            });
            amount_cards += 1;
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
}

impl Game {
    pub fn new() -> Self {
        Self {
            players: vec![],
            deck: vec![],
            discard_pile: vec![],
            current_player: 0,
            direction: 1,
        }
    }
    pub fn add_cards_to_deck(&mut self) {
        let mut deck_instance = Deck::new();
        deck_instance.add_cards_to_deck();
        self.deck = deck_instance.deck;
    }
}
