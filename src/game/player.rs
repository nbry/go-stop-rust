use crate::cards::active::ActiveCards;
use crate::cards::capture_pile::CapturePile;

pub struct Player {
    pub name: String,
    pub active: bool,
    pub captures: CapturePile,
    pub chips: i32,
    pub go: i32,
    pub shakes: i32,
    hand: ActiveCards,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name,
            active: false,
            captures: CapturePile::new(),
            chips: 100,
            go: 0,
            shakes: 0,
            hand: ActiveCards::new(),
        }
    }

    // fn pay_player(&mut self, &mut other_player: &Player) {}
}
