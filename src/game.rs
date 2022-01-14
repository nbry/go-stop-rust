pub mod player;
mod score;
pub mod settings;

use crate::cards::active::ActiveCards;
use crate::cards::deck::Deck;
use crate::game::player::Player;
use settings::HouseRules;
use std::io;

pub fn run() {
    let mut game = Game::new();
    game.add_players();
    game.deal_cards();
}

// new game *
//  game settings *
//  game state *
//  load deck *
// add players *
//  io: player's name *
//  player capture pile *
// last player cuts deck
// last player sees bottom card
// cards are dealt
// game loop begin:
//  print player hand and river
//  io: player's choice (choice is fixed pre-flip, for now)
//  match cards (if any)
//  flip card
//  check river state
//  capture cards
//  add to player's card collection
//  check and print score state
//  can player stop?
//  if go, update score state
// game loop ends:
// distribute chips
// end game if:
//  a player has captured all chips
//  someone gives up
//  game is manually ended

pub struct Game {
    players: Vec<Player>,
    whose_turn: usize,
    deck: Deck,
    river: ActiveCards,
    house_rules: HouseRules,
}

impl Game {
    pub fn new() -> Game {
        Game {
            players: Vec::new(),
            whose_turn: 0,
            deck: Deck::new(true),
            river: ActiveCards::new(),
            house_rules: HouseRules::new(),
        }
    }

    pub fn add_players(&mut self) {
        // TODO: Make configurable number of players
        const NUM_PLAYERS: i8 = 2;

        for i in 1..=NUM_PLAYERS {
            print!("\r");
            let mut player_name = String::new();
            println!("Enter name of Player {}", i);
            io::stdin().read_line(&mut player_name).expect("a string");
            self.add_player(player_name);
        }
    }

    fn add_player(&mut self, name: String) {
        self.players.push(Player::new(name));
    }

    pub fn deal_cards(&mut self) {
        /*
        TODO: Deal cards in traditional way
        TODO: Make configurable, based on number of players
        */
        const STARTING_NUM_CARDS_IN_HAND: i8 = 10;
        const STARTING_NUM_CARDS_IN_RIVER: i8 = 10;

        for player in self.players.iter_mut() {
            for _i in 0..STARTING_NUM_CARDS_IN_HAND {
                player.hand.add_card(self.deck.draw());
            }
        }

        for _i in 0..STARTING_NUM_CARDS_IN_RIVER {
            self.river.add_card(self.deck.draw());
        }
    }

    pub fn next_turn(&mut self) {
        if self.whose_turn == self.players.len() - 1 {
            self.whose_turn = 0;
        } else {
            self.whose_turn += 1;
        }
    }

    pub fn initiate_game_loop() {}
}
