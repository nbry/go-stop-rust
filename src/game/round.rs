use crate::game::cards::{active::ActiveCards, deck::Deck};

use super::player::Player;

#[derive(Debug)]
struct Round {
    deck: Deck,
    players: Vec<Player>,
    river: ActiveCards,
    turn: usize,
}

impl Round {
    pub fn deal_cards(&mut self, tap: bool) {
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
                self.river.add_card(self.deck.draw());
            }
            let num_cards = deal.pop().unwrap();

            for player in self.players.iter_mut() {
                for _i in 0..num_cards {
                    player
                        .active
                        .as_mut()
                        .unwrap()
                        .hand
                        .add_card(self.deck.draw());
                }
            }
        }
    }

    fn activate_all_players(&mut self) {
        for player in self.players.iter_mut() {
            player.activate();
        }

        println!("ALL PLAYERS ACTIVATED");
    }

    fn last_player(self) -> usize {
        if self.turn as i8 - 1 < 0 {
            return self.players.len() - 1;
        }

        self.turn - 1
    }
}

pub fn run_round(players: Vec<Player>) -> Vec<Player> {
    println!("ROUND STARTED");

    let mut round = Round {
        deck: Deck::new(true),
        players,
        river: ActiveCards::new(),
        turn: 0,
    };

    //TODO: Implement cut and tap
    round.deal_cards(false);

    println!("RIVER: {:#?}", round.river);
    println!("{:#?}", round.players);

    // round.determine_active_players()

    round.players
}
