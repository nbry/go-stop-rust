use crate::game::cards::active::ActiveCards;
use crate::game::cards::capture_pile::CapturePile;

/// Contains state for an active player in a round
#[derive(Debug)]
pub struct ActiveState {
    pub hand: ActiveCards,
    pub captures: CapturePile,
    pub go: i32,
    pub shakes: i32,
}

impl ActiveState {
    fn new() -> ActiveState {
        ActiveState {
            hand: ActiveCards::new(),
            captures: CapturePile::new(),
            go: 0,
            shakes: 0,
        }
    }
}

/// Player identity struct, persists through game.
/// Player is actively in play if active is not None.
#[derive(Debug)]
pub struct Player {
    pub id: usize,
    pub name: String,
    pub chips: i32,
    pub active: Option<ActiveState>,
}

impl Player {
    pub fn new(name: String, id: usize) -> Player {
        Player {
            id,
            name,
            chips: 100,
            active: None,
        }
    }

    pub fn activate(&mut self) {
        self.active = Some(ActiveState::new());
    }

    pub fn deactivate(&mut self) {
        self.active = None;
    }
}
