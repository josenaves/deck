#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        // List of suits - hearts, diamonds, clubs, spades
        let suits = ["hearts", "diamonds", "clubs", "spades"];

        // List of values - 2, 3, 4, 5, 6, 7, 8, 9, 10, J, Q, K, A 
        let values = ["2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"];

        let mut cards = vec![];

        for suit in suits {
            for value in values {
                // Create a card and add it to the deck
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        Deck { cards }
    }
}

fn main() {
    let deck = Deck::new();
    println!("Here is your deck: {:#?}", deck);
}
