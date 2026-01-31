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
    pub deck: Deck,
    pub discard_pile: Vec<Card>,
    pub current_player: i32,
    pub direction: i8,
}

pub struct Deck {
    pub deck: Vec<Card>,
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

    fn deal_8_cards(&mut self) -> Vec<Card> {
        let mut hand: Vec<Card> = vec![];
        let mut dealt_cards = 0;
        while dealt_cards != 8 {
            if let Some(card) = self.deck.pop() {
                hand.push(card);
            }
            dealt_cards += 1;
        }
        hand
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            players: vec![],
            deck: Deck::new(),
            discard_pile: vec![],
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
            let player = Player { hand };
            self.players.push(player);
        }
    }
}
