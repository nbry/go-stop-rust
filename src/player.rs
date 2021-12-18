use crate::active_cards::ActiveCards;
use crate::capture_pile::CapturePile;

pub struct Player {
    name: String,
    chips: i32,
    active: bool,
    hand: ActiveCards,
    captures: CapturePile,
    go: i32,
    shakes: i32,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name,
            go: 0,
            active: false,
            captures: CapturePile::new(),
            hand: ActiveCards::new(),
            chips: 100,
            shakes: 0,
        }
    }

    fn get_score(&mut self) -> i32 {
        let mut score = self.captures.points;

        if self.captures.seven_animals {
            score *= 2;
        }

        score
    }

    // fn pay_player(&mut self, &mut other_player: &Player) {}
}
