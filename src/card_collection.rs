use crate::card::{Card, Category, Special, Suit};

#[derive(Debug)]
pub struct CardCollection {
    pub points: i32,
    sep_animal_is_junk: bool,
    animals: Vec<Card>,
    brights: Vec<Card>,
    junk: Vec<Card>,
    junk_animal: Vec<Card>,
    ribbons: Vec<Card>,
}

impl CardCollection {
    pub fn new() -> CardCollection {
        CardCollection {
            points: 0,
            sep_animal_is_junk: false,
            animals: Vec::new(),
            brights: Vec::new(),
            junk: Vec::new(),
            junk_animal: Vec::new(),
            ribbons: Vec::new(),
        }
    }

    pub fn add_card(&mut self, card: Card) {
        match card.category {
            Category::Animal => self.animals.push(card),
            Category::Bright => self.brights.push(card),
            Category::Junk => self.junk.push(card),
            Category::Ribbon => self.ribbons.push(card),
            Category::JunkAnimal => self.junk_animal.push(card),
        }
    }

    pub fn set_junk_animal_is_junk(&mut self, is_junk: bool) {
        self.sep_animal_is_junk = is_junk;
    }

    pub fn update_score(&mut self) {
        let mut points = 0;
        points += self.count_animal_points();
        points += self.count_brights_points();
        points += self.count_junk_points();
        points += self.count_ribbons_points();
        self.points = points
    }

    fn count_animal_points(&self) -> i32 {
        let mut animal_points = 0;
        let mut animals_amt = 0;
        let mut birds_amt = 0;
        let mut december_bird_amt = 0;

        // This probably needs refactoring, for house rules (may|animal)
        if !self.sep_animal_is_junk {
            animals_amt += self.junk_animal.len() as i32;
        }

        for card in &self.animals {
            animals_amt += 1;
            if matches!(card.special, Special::Bird) {
                if matches!(card.suit, Suit::Dec) {
                    december_bird_amt += 1;
                } else {
                    birds_amt += 1;
                }
            }
        }
        if animals_amt >= 5 {
            animal_points += animals_amt - 4;
        }
        if birds_amt == 3 {
            if december_bird_amt == 1 {
                animal_points += 10;
            } else {
                animal_points += 5;
            }
        }
        animal_points
    }

    fn count_brights_points(&self) -> i32 {
        let mut brights_points = 0;
        let mut brights_amt = 0;
        let mut december_bright = 0;

        for card in &self.brights {
            if matches!(card.suit, Suit::Dec) {
                december_bright += 1;
            } else {
                brights_amt += 1;
            }
        }

        match brights_amt + december_bright {
            3 => brights_points = 3 - december_bright,
            4 => brights_points = 4,
            5 => brights_points = 15,
            _ => (),
        }
        brights_points
    }

    fn count_junk_points(&self) -> i32 {
        let mut junk_points = 0;
        let mut junk_amt = 0;

        if self.sep_animal_is_junk {
            junk_amt += self.junk_animal.len() as i32 * 2;
        }
        for card in &self.junk {
            match card.special {
                Special::Double => junk_amt += 2,
                Special::None => junk_amt += 1,
                _ => (),
            }
        }
        if junk_amt >= 10 {
            junk_points += junk_amt - 9;
        }

        junk_points
    }

    fn count_ribbons_points(&self) -> i32 {
        let mut ribbons_points = 0;
        let mut ribbons_amt = 0;
        let mut red_labels_amt = 0;
        let mut blue_labels_amt = 0;
        let mut blank_reds_amt = 0;

        for card in &self.ribbons {
            ribbons_amt += 1;
            if !matches!(card.special, Special::None) {
                match card.special {
                    Special::RedLabel => red_labels_amt += 1,
                    Special::BlueLabel => blue_labels_amt += 1,
                    Special::BlankRed => blank_reds_amt += 1,
                    _ => (),
                }
            }
        }
        if ribbons_amt >= 5 {
            ribbons_points += ribbons_amt - 4;
        }
        if red_labels_amt == 3 {
            ribbons_points += 3;
        }
        if blue_labels_amt == 3 {
            ribbons_points += 3;
        }
        if blank_reds_amt == 3 {
            ribbons_points += 3;
        }
        ribbons_points
    }
}
