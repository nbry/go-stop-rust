pub mod player;
mod score;
pub mod settings;

use crate::cards::active::ActiveCards;
use crate::cards::deck::Deck;
use crate::game::player::Player;
use settings::ScoreSettings;
use std::io;

pub fn run() {
    let mut game_state = GameState::new();

    println!("Add Player 1");
    let mut player_one = String::new();
    io::stdin().read_line(&mut player_one).expect("a string");
    game_state.add_player(player_one);

    println!("Add Player 2");
    let mut player_two = String::new();
    io::stdin().read_line(&mut player_two).expect("a string");
    game_state.add_player(player_two);
}

// new game
//  game settings
//  game state
//  load deck
// add players
//  io: player's name
//  player capture pile
// load river and deal cards
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

pub struct GameState {
    players: Vec<Player>,
    whose_turn: usize,
    deck: Deck,
    river: ActiveCards,
    score_settings: ScoreSettings,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            players: Vec::new(),
            whose_turn: 0,
            deck: Deck::new(true),
            river: ActiveCards::new(),
            score_settings: ScoreSettings::new(),
        }
    }

    pub fn add_player(&mut self, name: String) {
        self.players.push(Player::new(name));
    }

    pub fn next_turn(&mut self) {
        self.whose_turn = if self.whose_turn == self.players.len() - 1 {
            0
        } else {
            self.whose_turn + 1
        }

        // if self.whose_turn == self.players.len() - 1 {
        //     self.whose_turn = 0;
        // } else {
        //     self.whose_turn += 1;
        // }
    }

    pub fn deal_cards() {}

    pub fn initiate_game_loop() {}
}
