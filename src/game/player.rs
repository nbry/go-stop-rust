use std::io;

use crate::game::cards::active::ActiveCards;
use crate::game::cards::capture_pile::CapturePile;
use crate::game::cards::deck::Deck;

/// Contains state for an active player in a round
#[derive(Debug)]
pub struct ActiveState {
    hand: ActiveCards,
    captures: CapturePile,
    go: i32,
    shakes: i32,
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
    fn new(name: String, id: usize) -> Player {
        Player {
            id,
            name,
            chips: 100,
            active: None,
        }
    }
}

/// Responsible for all actions regarding a game's players
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
        //TODO: Implement 4-5 player games
        const MIN_PLAYERS: usize = 2;
        const MAX_PLAYERS: usize = 3;
        println!("Enter 2-3 Players");

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

    ///Deal cards in traditional way.
    pub fn deal_cards(&mut self, tap: bool, deck: &mut Deck, river: &mut ActiveCards) {
        self.activate_all_players();

        let mut deal = match self.players.len() {
            2 => {
                if tap {
                    vec![10, 8]
                } else {
                    vec![5, 4, 5, 4]
                }
            }
            _ => {
                if tap {
                    vec![7, 6]
                } else {
                    vec![3, 3, 4, 3]
                }
            }
        };

        while deal.len() != 0 {
            let num_cards = deal.pop().unwrap();

            for _i in 0..num_cards {
                river.add_card(deck.draw());
            }
            let num_cards = deal.pop().unwrap();

            for player in self.players.iter_mut() {
                for _i in 0..num_cards {
                    player.active.as_mut().unwrap().hand.add_card(deck.draw());
                }
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

    /// Get the index of the last player in the round
    pub fn last_player(self) -> usize {
        if self.turn as i8 - 1 < 0 {
            return self.players.len() - 1 as usize;
        }

        self.turn - 1
    }

    fn activate_all_players(&mut self) {
        for player in self.players.iter_mut() {
            player.active = Some(ActiveState::new());
        }

        println!("ALL PLAYERS ACTIVATED");
    }
}
