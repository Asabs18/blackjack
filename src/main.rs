use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io::{self, Write};

// --- Model ---
struct Card {
    rank: u8,
    suit: &'static str,
}

struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Self {
        let suits = ["Hearts", "Diamonds", "Spades", "Clubs"];
        let mut cards = Vec::new();

        for &suit in &suits {
            for rank in 1..=13 {
                cards.push(Card { rank, suit });
            }
        }

        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal_card(&mut self) -> Card {
        self.cards.pop().expect("The deck is empty!")
    }
}

struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new() -> Self {
        Hand { cards: Vec::new() }
    }

    fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn calculate_hand_total(&self) -> u32 {
        let mut total = 0;
        let mut ace_count = 0;

        for card in &self.cards {
            match card.rank {
                1 => {
                    total += 11;
                    ace_count += 1;
                }
                11..=13 => total += 10,
                _ => total += card.rank as u32,
            }
        }

        while total > 21 && ace_count > 0 {
            total -= 10;
            ace_count -= 1;
        }

        total
    }

    fn display(&self, viewer: &dyn CardView) {
        let output = self
            .cards
            .iter()
            .map(|card| viewer.draw(card))
            .collect::<Vec<String>>()
            .join(", ");

        println!("Hand: {}", output);
    }
}

// --- View ---
trait CardView {
    fn draw(&self, card: &Card) -> String;
}

struct CardAlphaViewer;
struct CardGlyphViewer;

impl CardView for CardAlphaViewer {
    fn draw(&self, card: &Card) -> String {
        let rank = match card.rank {
            1 => "Ace".to_string(),
            11 => "Jack".to_string(),
            12 => "Queen".to_string(),
            13 => "King".to_string(),
            _ => card.rank.to_string(),
        };
        format!("{} of {}", rank, card.suit)
    }
}

impl CardView for CardGlyphViewer {
    fn draw(&self, card: &Card) -> String {
        let rank = match card.rank {
            1 => "A".to_string(),
            11 => "J".to_string(),
            12 => "Q".to_string(),
            13 => "K".to_string(),
            _ => card.rank.to_string(),
        };

        let glyph = match card.suit {
            "Hearts" => "♥",
            "Diamonds" => "♦",
            "Spades" => "♠",
            "Clubs" => "♣",
            _ => "?",
        };

        format!("{} of {}", rank, glyph)
    }
}

// --- Controller ---
trait GameController {
    fn run(&mut self) -> bool; // Run the game loop, handling all logic
}

struct Controller {
    deck: Deck,
    player_hand: Hand,
    dealer_hand: Hand,
    viewer: Box<dyn CardView>,
}

impl Controller {
    fn new(viewer: Box<dyn CardView>) -> Self {
        Controller {
            deck: Deck::new(),
            player_hand: Hand::new(),
            dealer_hand: Hand::new(),
            viewer: viewer,
        }
    }

    fn setup_game(&mut self) {
        self.deck.shuffle();
        self.player_hand = Hand::new();
        self.dealer_hand = Hand::new();
        self.deal_initial_hands();
    }

    fn deal_initial_hands(&mut self) {
        for _ in 0..2 {
            self.dealer_hand.add(self.deck.deal_card());
            self.player_hand.add(self.deck.deal_card());
        }
    }

    fn player_turn(&mut self) -> bool {
        loop {
            println!(
                "Player's hand total: {}",
                self.player_hand.calculate_hand_total()
            );
            self.player_hand.display(&*self.viewer);

            if self.player_hand.calculate_hand_total() > 21 {
                println!("Player busts! Dealer wins.");
                return false;
            }

            println!("Do you want to (h)it or (s)tand?");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let choice = input.trim();

            if choice == "h" {
                self.player_hand.add(self.deck.deal_card());
            } else if choice == "s" {
                break;
            } else {
                println!("Invalid choice, please enter 'h' or 's'.");
            }
        }
        true
    }

    fn dealer_turn(&mut self) {
        while self.dealer_hand.calculate_hand_total() < 17 {
            self.dealer_hand.add(self.deck.deal_card());
        }
    }

    fn determine_winner(&self) {
        let dealer_total = self.dealer_hand.calculate_hand_total();
        let player_total = self.player_hand.calculate_hand_total();

        println!("Dealer's hand total: {}", dealer_total);
        self.dealer_hand.display(&*self.viewer);
        println!("Player's hand total: {}", player_total);
        self.player_hand.display(&*self.viewer);

        if player_total > 21 {
            println!("Player busts! Dealer wins.");
        } else if dealer_total > 21 {
            println!("Dealer busts! Player wins.");
        } else if player_total > dealer_total {
            println!("Player wins!");
        } else if dealer_total > player_total {
            println!("Dealer wins!");
        } else {
            println!("It's a tie!");
        }
    }

    fn play_game(&mut self) {
        self.setup_game();

        if self.player_turn() {
            self.dealer_turn();
            self.determine_winner();
        }
    }

    fn play_again() -> bool {
        print!("\nDo you want to play again? (y/n): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim() == "y"
    }
}

impl GameController for Controller {
    fn run(&mut self) -> bool {
        self.play_game();
        Controller::play_again()
    }
}

fn main() {
    let is_glyph_view = true; // Toggle this to switch between glyph and alpha viewer

    let viewer: Box<dyn CardView> = if is_glyph_view {
        Box::new(CardGlyphViewer)
    } else {
        Box::new(CardAlphaViewer)
    };

    let mut controller = Controller::new(viewer);

    loop {
        if !controller.run() {
            break;
        }
    }
}
