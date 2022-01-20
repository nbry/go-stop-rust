use std::io;

use crate::cards::active::ActiveCards;
use crate::cards::capture_pile::CapturePile;

#[derive(Debug)]
pub struct Players {
    players: Vec<Player>,
    dealer: usize,
    turn: usize,
}

impl Players {
    pub fn new() -> Players {
        Players {
            players: Vec::new(),
            dealer: 0,
            turn: 0,
        }
    }
    pub fn add_players(&mut self) {
        const MIN_PLAYERS: usize = 2;
        const MAX_PLAYERS: usize = 4;
        println!("Enter 2-4 Players");

        while self.players.len() < MAX_PLAYERS {
            let mut entry = String::new();

            println!("Enter name of Player {}", self.players.len() + 1);
            io::stdin().read_line(&mut entry).expect("a string");

            if entry.trim().eq("Done") {
                if self.players.len() < MIN_PLAYERS {
                    println!("Need at least 2 players");
                } else {
                    break;
                }
            } else {
                self.add_player(entry.trim().to_string());
            }
        }
    }

    pub fn add_player(&mut self, mut name: String) {
        if name.eq("") {
            name = format!("Player {}", self.players.len() + 1);
        } else {
            let mut duplicate_name_count = 0;

            for player in self.players.iter_mut() {
                if name.eq(&player.name) || name.eq(&player.name[0..player.name.len() - 3]) {
                    duplicate_name_count += 1;
                }
            }

            if duplicate_name_count > 0 {
                name = format!("{}({})", name, duplicate_name_count);
            }
        }

        println!("Adding {}!", name);
        self.players.push(Player::new(name, self.players.len()));
    }

    pub fn set_dealer(&mut self) {
        self.dealer = self.turn;
    }

    pub fn cycle(&mut self, action: AllPlayersAction) {
        for player in self.players.iter_mut() {
            match action {
                AllPlayersAction::RoundReset => player.round_reset(),
            }
        }
    }

    pub fn a_player_has_zero_chips(&self) -> bool {
        for player in &self.players {
            if player.chips == 0 {
                return true;
            }
        }
        false
    }
}

pub enum AllPlayersAction {
    RoundReset,
}

#[derive(Debug)]
pub struct Player {
    pub id: usize,
    pub name: String,
    pub chips: i32,
    pub hand: Option<ActiveCards>,
    pub captures: CapturePile,
    pub go: i32,
    pub shakes: i32,
}

impl Player {
    fn new(name: String, id: usize) -> Player {
        Player {
            id,
            name,
            captures: CapturePile::new(),
            chips: 100,
            go: 0,
            shakes: 0,
            hand: None,
        }
    }

    fn round_reset(&mut self) {
        self.hand = None;
        self.captures = CapturePile::new();
        self.go = 0;
        self.shakes = 0;
    }

    // pub fn play_card(suit: Suit) {}

    // fn pay_player(&mut self, &mut other_player: &Player) {}
}
