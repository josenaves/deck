use rand::{rng, seq::SliceRandom};

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

    fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(
            self.cards.len() - num_cards
        )
    }
}

fn main() {
    let mut deck= Deck::new();

    deck.shuffle();
    println!("Here is your deck: {:#?}", deck);

    // need to add error handling for when the user tries to deal more cards than are in the deck
    let cards = deck.deal(4);
    println!("Here is your hand: {:#?}", cards);

    println!("Here is your deck after deal: {:#?}", deck);
}
