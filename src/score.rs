use crate::card::{Card, Category, Special, Suit};

#[derive(Default, Debug)]
pub struct Score {
    total: i32,
    junk: i32,
    double_junk: i32,
    brights: i32,
    december_bright: i32,
    animals: i32,
    junk_animal: i32,
    birds: i32,
    december_bird: i32,
    ribbons: i32,
    red_labels: i32,
    blue_labels: i32,
    blank_red: i32,
    go: i32,
    is_updated: bool,
    sep_animal_is_junk: bool,
}

impl Score {
    pub fn new() -> Score {
        Score {
            sep_animal_is_junk: true,
            ..Default::default()
        }
    }

    pub fn get_total(&mut self) -> i32 {
        if !self.is_updated {
            self.update_score();
        }
        self.total
    }

    pub fn add_card(&mut self, card: Card) {
        self.is_updated = false;
        match card.category {
            Category::Animal => self.add_animal(card),
            Category::Bright => self.add_bright(card),
            Category::Ribbon => self.add_ribbon(card),
            Category::Junk => self.add_junk(card),
            Category::JunkAnimal => self.junk_animal += 1,
        }
    }

    pub fn set_junk_animal_is_junk(&mut self, is_junk: bool) {
        self.sep_animal_is_junk = is_junk;
    }

    fn update_score(&mut self) {
        let mut points = 0;

        // JUNK
        let raw_junk_amt = self.double_junk * 2 + self.junk;
        if raw_junk_amt >= 10 {
            points += raw_junk_amt - 9;
            if self.junk_animal > 0 && self.sep_animal_is_junk {
                points += 2;
            }
        }

        // BRIGHT
        let raw_bright_amt = self.brights + self.december_bright;
        match raw_bright_amt {
            3 => points += 3 - self.december_bright,
            4 => points += 4,
            5 => points += 15,
            _ => (),
        }

        // RIBBON
        if self.ribbons >= 5 {
            points += self.ribbons - 4;
        }
        if self.red_labels == 3 {
            points += 3;
        }
        if self.blue_labels == 3 {
            points += 3;
        }
        if self.blank_red == 3 {
            points += 3;
        }

        // ANIMAL
        if self.animals >= 5 {
            points += self.animals - 4;
            if self.junk_animal > 0 && !self.sep_animal_is_junk {
                points += 1;
            }
        }
        if self.birds == 3 {
            if self.december_bird == 1 {
                points += 10;
            } else {
                points += 5;
            }
        }

        if self.go < 3 {
            points += self.go;
        }

        self.total = points;
        self.is_updated = true;
    }

    fn add_animal(&mut self, card: Card) {
        self.animals += 1;

        if matches!(card.special, Special::Bird) {
            if matches!(card.suit, Suit::Dec) {
                self.december_bird += 1;
            } else {
                self.birds += 1;
            }
        }
    }

    fn add_bright(&mut self, card: Card) {
        if matches!(card.suit, Suit::Dec) {
            self.december_bright += 1;
        } else {
            self.brights += 1;
        }
    }

    fn add_ribbon(&mut self, card: Card) {
        self.ribbons += 1;
        if !matches!(card.special, Special::None) {
            match card.special {
                Special::RedLabel => self.red_labels += 1,
                Special::BlueLabel => self.blue_labels += 1,
                Special::BlankRed => self.blank_red += 1,
                _ => (),
            }
        }
    }

    fn add_junk(&mut self, card: Card) {
        if matches!(card.special, Special::Double) {
            self.double_junk += 1;
        } else {
            self.junk += 1;
        }
    }
}
