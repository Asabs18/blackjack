use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io::{self, Write};

/// The `Model` trait defines a common interface for data models in an application.
///
/// The `Model` trait is implemented for any type `T`, and provides two methods:
/// - `get_data(&self) -> &T`: Returns a reference to the underlying data.
/// - `set_data(&mut self, data: T)`: Updates the underlying data.
///
/// This trait allows data models to be used interchangeably throughout an application,
/// without needing to know the specific implementation details of each model.
trait Model<T> {
    fn get_data(&self) -> &T;
    fn set_data(&mut self, data: T);
}

/// The `View` trait defines a common interface for rendering a data model as a string.
///
/// The `View` trait is implemented for any type `T`, and provides one method:
/// - `draw(&self, model: &T) -> String`: Renders the given data model as a string.
///
/// This trait allows different views to be used to display the same data model,
/// without needing to know the specific implementation details of each view.
trait View<T> {
    fn draw(&self, model: &T) -> String;
}

/// The `Controller` trait defines a common interface for controlling the application flow.
///
/// The `Controller` trait is implemented for any type `T`, and provides one method:
/// - `run(&mut self) -> bool`: Executes the controller logic, returning a boolean
///   indicating whether the application should continue running.
///
/// This trait allows different controllers to be used to drive the application,
/// without needing to know the specific implementation details of each controller.
trait Controller<T> {
    fn run(&mut self) -> bool; // Return bool indicating whether to continue
}

// --- Card and Deck Models ---

/// A playing card with a rank (1-13) and suit.
///
/// The `Card` struct represents a standard playing card with a numeric rank (1-13, representing Ace through King) and a suit (Hearts, Diamonds, Spades, or Clubs).
/// The `Model` trait is implemented for `Card`, allowing it to be used as a data model in a larger application.
struct Card {
    rank: u8,
    suit: &'static str,
}

impl Model<Card> for Card {
    fn get_data(&self) -> &Card {
        self
    }

    fn set_data(&mut self, data: Card) {
        *self = data;
    }
}

/// The `Deck` struct represents a standard deck of 52 playing cards.
///
/// The `Deck` struct contains a `Vec` of `Card` instances, representing the cards in the deck.
/// The `new()` method creates a new deck with all 52 cards, the `shuffle()` method shuffles the deck,
/// and the `deal_card()` method removes and returns the top card from the deck.
/// The `Deck` struct implements the `Model` trait, allowing it to be used as a data model in a larger application.
struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    /// Creates a new deck of 52 shuffled playing cards.
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

    /// Shuffles the deck using a random number generator.
    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    /// Deals the top card from the deck and removes it.
    fn deal_card(&mut self) -> Card {
        self.cards.pop().expect("The deck is empty!")
    }
}

impl Model<Vec<Card>> for Deck {
    fn get_data(&self) -> &Vec<Card> {
        &self.cards
    }

    fn set_data(&mut self, data: Vec<Card>) {
        self.cards = data;
    }
}

// --- Hand Model ---

/// The `Hand` struct represents a player's hand of cards in a card game.
///
/// The `Hand` struct contains a `Vec` of `Card` instances, representing the cards in the player's hand.
/// The `new()` method creates a new empty hand, the `add()` method adds a card to the hand,
/// the `calculate_hand_total()` method calculates the total value of the cards in the hand,
/// and the `display()` method displays the cards in the hand using a provided `View` implementation.
/// The `Hand` struct implements the `Model` trait, allowing it to be used as a data model in a larger application.
struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    /// Creates a new empty hand.
    fn new() -> Self {
        Hand { cards: Vec::new() }
    }

    /// Adds a card to the player's hand.
    fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    /// Calculates the total value of the hand, adjusting for Ace cards.
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

        // Adjust for Aces if the total is over 21
        while total > 21 && ace_count > 0 {
            total -= 10;
            ace_count -= 1;
        }

        total
    }

    /// Displays the hand using the specified viewer.
    fn display(&self, viewer: &dyn View<Hand>) {
        let output = viewer.draw(self);
        println!("Hand: {}", output);
    }
}

impl Model<Vec<Card>> for Hand {
    fn get_data(&self) -> &Vec<Card> {
        &self.cards
    }

    fn set_data(&mut self, data: Vec<Card>) {
        self.cards = data;
    }
}

// --- View Implementations ---

/// A viewer implementation that displays the cards in a hand using their alphabetic rank names.
///
/// This viewer is used to display the cards in a hand in a human-readable format, using the
/// alphabetic rank names (e.g. "Ace", "Jack", "Queen", "King") instead of the numeric ranks.
/// The suit of each card is also displayed.
struct CardAlphaViewer;

impl View<Hand> for CardAlphaViewer {
    fn draw(&self, model: &Hand) -> String {
        model
            .get_data()
            .iter()
            .map(|card| {
                let rank = match card.rank {
                    1 => "Ace".to_string(),
                    11 => "Jack".to_string(),
                    12 => "Queen".to_string(),
                    13 => "King".to_string(),
                    _ => card.rank.to_string(),
                };
                format!("{} of {}", rank, card.suit)
            })
            .collect::<Vec<String>>()
            .join(", ")
    }
}

/// A viewer implementation that displays the cards in a hand using their rank glyphs and suit symbols.
///
/// This viewer is used to display the cards in a hand in a compact, graphical format, using the
/// rank glyphs (e.g. "A", "J", "Q", "K") and suit symbols (♥, ♦, ♠, ♣) instead of the alphabetic
/// rank names and suit names. This provides a more concise and visually appealing representation
/// of the cards in the hand.
struct CardGlyphViewer;

impl View<Hand> for CardGlyphViewer {
    fn draw(&self, model: &Hand) -> String {
        model
            .get_data()
            .iter()
            .map(|card| {
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
            })
            .collect::<Vec<String>>()
            .join(", ")
    }
}

// --- Game Controller ---

/// A game controller that manages the game logic and flow for a card game.
///
/// The `GameController` struct is responsible for managing the game state, including the deck, player hand, and dealer hand. It also handles the game flow, such as dealing the initial hands, allowing the player to hit or stand, and determining the winner.
///
/// The `GameController` uses a `View` implementation to display the cards in the player's and dealer's hands. This allows the game to be displayed in different formats, such as using card glyphs or alphabetic rank and suit names.
///
/// The `GameController` provides a `run()` method that encapsulates the entire game loop, allowing the game to be easily played and restarted.
struct GameController {
    deck: Deck,
    player_hand: Hand,
    dealer_hand: Hand,
    viewer: Box<dyn View<Hand>>,
}

impl GameController {
    /// Creates a new game controller with the specified viewer.
    fn new(viewer: Box<dyn View<Hand>>) -> Self {
        GameController {
            deck: Deck::new(),
            player_hand: Hand::new(),
            dealer_hand: Hand::new(),
            viewer,
        }
    }

    /// Deals the initial hands for both the player and the dealer.
    fn deal_initial_hands(&mut self) {
        self.deck.shuffle();
        self.player_hand.add(self.deck.deal_card());
        self.dealer_hand.add(self.deck.deal_card());
        self.player_hand.add(self.deck.deal_card());
        self.dealer_hand.add(self.deck.deal_card());
    }

    /// Prompts the player to either hit or stand, and processes their choice.
    fn player_turn(&mut self) {
        loop {
            self.player_hand.display(&*self.viewer);
            println!("Your total: {}", self.player_hand.calculate_hand_total());
            println!("Do you want to (h)it or (s)tand?");
            let mut choice = String::new();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut choice).unwrap();
            match choice.trim().to_lowercase().as_str() {
                "h" => {
                    self.player_hand.add(self.deck.deal_card());
                    if self.player_hand.calculate_hand_total() > 21 {
                        println!("You bust! Your total is over 21.");
                        break;
                    }
                }
                "s" => break,
                _ => println!("Invalid choice, please choose 'h' to hit or 's' to stand."),
            }
        }
    }

    /// Plays the dealer's turn, where the dealer will automatically hit until their total is at least 17.
    fn dealer_turn(&mut self) {
        self.dealer_hand.display(&*self.viewer);
        println!(
            "Dealer's total: {}",
            self.dealer_hand.calculate_hand_total()
        );
        while self.dealer_hand.calculate_hand_total() < 17 {
            println!("Dealer hits...");
            self.dealer_hand.add(self.deck.deal_card());
            self.dealer_hand.display(&*self.viewer);
            println!(
                "Dealer's total: {}",
                self.dealer_hand.calculate_hand_total()
            );
        }
    }

    /// Determines the winner of the game based on the final totals of the player's and dealer's hands.
    fn determine_winner(&self) {
        let player_total = self.player_hand.calculate_hand_total();
        let dealer_total = self.dealer_hand.calculate_hand_total();

        if player_total > 21 {
            println!("You bust! Dealer wins.");
        } else if dealer_total > 21 {
            println!("Dealer busts! You win.");
        } else if player_total > dealer_total {
            println!("You win!");
        } else if player_total < dealer_total {
            println!("Dealer wins.");
        } else {
            println!("It's a tie!");
        }
    }
}

/// Runs the entire game, including the player's turn, dealer's turn, and winner determination.
///
/// The `run()` method encapsulates the game flow by calling methods to handle each phase of the game.
impl Controller<Hand> for GameController {
    fn run(&mut self) -> bool {
        self.deal_initial_hands();
        self.player_turn();
        self.dealer_turn();
        self.determine_winner();

        // Reset hands for the next game
        self.player_hand = Hand::new(); // Re-initialize the player's hand
        self.dealer_hand = Hand::new(); // Re-initialize the dealer's hand

        // Ask the user if they want to play again
        println!("Do you want to play again? (y/n)");
        let mut choice = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut choice).unwrap();
        choice.trim().to_lowercase() == "y"
    }
}

/// The main entry point of the application.
///
/// This function sets up the game controller with either a glyph or alpha viewer,
/// and then enters a loop where a new hand is created and the game is played.
/// The loop continues until the user chooses not to play again.
fn main() {
    let is_glyph_view = true; // Toggle this to switch between glyph and alpha viewer

    let viewer: Box<dyn View<Hand>> = if is_glyph_view {
        Box::new(CardGlyphViewer)
    } else {
        Box::new(CardAlphaViewer)
    };

    let mut controller = GameController::new(viewer);

    loop {
        let mut hand = Hand::new(); // Create the hand model
        if !controller.run() {
            // If play_again returns false, break the loop
            break;
        }
    }
}
