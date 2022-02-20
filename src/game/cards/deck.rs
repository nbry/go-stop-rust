use crate::game::cards::card::{Card, Category, Special, Suit};
use rand::Rng;

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new(shuffle: bool) -> Self {
        let mut new_deck = Deck { cards: vec![] };
        new_deck.load();
        if shuffle {
            new_deck.shuffle();
        }

        new_deck
    }

    pub fn draw(&mut self) -> Card {
        self.cards.pop().expect("Deck has no cards remaining")
    }

    fn shuffle(&mut self) {
        for i in (0..47).rev() {
            let j = rand::thread_rng().gen_range(0..i + 1);
            self.swap(i, j);
        }
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.cards.swap(i, j);
    }

    fn load(&mut self) {
        self.cards
            .push(Card::new(Suit::Jan, Category::Bright, Special::None));
        self.cards
            .push(Card::new(Suit::Jan, Category::Ribbon, Special::RedLabel));
        self.cards
            .push(Card::new(Suit::Jan, Category::Junk, Special::None));
        self.cards
            .push(Card::new(Suit::Jan, Category::Junk, Special::None));

        self.cards
            .push(Card::new(Suit::Feb, Category::Animal, Special::Bird));
        self.cards
            .push(Card::new(Suit::Feb, Category::Ribbon, Special::RedLabel));
        self.cards
            .push(Card::new(Suit::Feb, Category::Junk, Special::None));
        self.cards
            .push(Card::new(Suit::Feb, Category::Junk, Special::None));

        self.cards
            .push(Card::new(Suit::Mar, Category::Bright, Special::None));
        self.cards
            .push(Card::new(Suit::Mar, Category::Ribbon, Special::RedLabel));
        self.cards
            .push(Card::new(Suit::Mar, Category::Junk, Special::None));
        self.cards
            .push(Card::new(Suit::Mar, Category::Junk, Special::None));

        self.cards
            .push(Card::new(Suit::Apr, Category::Animal, Special::Bird));
        self.cards
            .push(Card::new(Suit::Apr, Category::Ribbon, Special::BlankRed));
        self.cards
            .push(Card::new(Suit::Apr, Category::Junk, Special::None));
        self.cards
            .push(Card::new(Suit::Apr, Category::Junk, Special::None));

        self.cards
            .push(Card::new(Suit::May, Category::Animal, Special::None));
        self.cards
            .push(Card::new(Suit::May, Category::Ribbon, Special::BlankRed));
        self.cards
            .push(Card::new(Suit::May, Category::Junk, Special::None));
        self.cards
            .push(Card::new(Suit::May, Category::Junk, Special::None));

        self.cards
            .push(Card::new(Suit::Jun, Category::Animal, Special::None));
        self.cards
            .push(Card::new(Suit::Jun, Category::Ribbon, Special::BlueLabel));
        self.cards
            .push(Card::new(Suit::Jun, Category::Junk, Special::None));
        self.cards
            .push(Card::new(Suit::Jun, Category::Junk, Special::None));

        self.cards
            .push(Card::new(Suit::Jul, Category::Animal, Special::None));
        self.cards
            .push(Card::new(Suit::Jul, Category::Ribbon, Special::BlankRed));
        self.cards
            .push(Card::new(Suit::Jul, Category::Junk, Special::None));
        self.cards
            .push(Card::new(Suit::Jul, Category::Junk, Special::None));

        self.cards
            .push(Card::new(Suit::Aug, Category::Bright, Special::None));
        self.cards
            .push(Card::new(Suit::Aug, Category::Animal, Special::Bird));
        self.cards
            .push(Card::new(Suit::Aug, Category::Junk, Special::None));
        self.cards
            .push(Card::new(Suit::Aug, Category::Junk, Special::None));

        self.cards
            .push(Card::new(Suit::Sep, Category::JunkAnimal, Special::None));
        self.cards
            .push(Card::new(Suit::Sep, Category::Ribbon, Special::BlueLabel));
        self.cards
            .push(Card::new(Suit::Sep, Category::Junk, Special::None));
        self.cards
            .push(Card::new(Suit::Sep, Category::Junk, Special::None));

        self.cards
            .push(Card::new(Suit::Oct, Category::Animal, Special::None));
        self.cards
            .push(Card::new(Suit::Oct, Category::Ribbon, Special::BlueLabel));
        self.cards
            .push(Card::new(Suit::Oct, Category::Junk, Special::None));
        self.cards
            .push(Card::new(Suit::Oct, Category::Junk, Special::None));

        self.cards
            .push(Card::new(Suit::Nov, Category::Bright, Special::None));
        self.cards
            .push(Card::new(Suit::Nov, Category::Junk, Special::Double));
        self.cards
            .push(Card::new(Suit::Nov, Category::Junk, Special::None));
        self.cards
            .push(Card::new(Suit::Nov, Category::Junk, Special::None));

        self.cards
            .push(Card::new(Suit::Dec, Category::Bright, Special::None));
        self.cards
            .push(Card::new(Suit::Dec, Category::Ribbon, Special::None));
        self.cards
            .push(Card::new(Suit::Dec, Category::Animal, Special::Bird));
        self.cards
            .push(Card::new(Suit::Dec, Category::Junk, Special::Double));
    }
}
