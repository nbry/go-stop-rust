pub struct Card {
    pub suit: Suit,
    pub category: Category,
    pub special: Special,
}
impl Card {
    pub fn new(suit: Suit, category: Category, special: Special) -> Self {
        Card {
            suit,
            category,
            special,
        }
    }
}

pub trait Draw {
    fn draw(&mut self) -> Card;
}

pub enum Suit {
    Jan,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec,
}

pub enum Category {
    Bright,
    Ribbon,
    Animal,
    Junk,
    JunkAnimal,
}

pub enum Special {
    RedLabel,
    BlueLabel,
    BlankRed,
    Bird,
    Double,
    None,
}
