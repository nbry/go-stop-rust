pub mod player;
mod round;
mod score;
pub mod settings;

use player::Player;
use round::Round;
use settings::Settings;
use std::{collections::VecDeque, io};

pub fn run() {
    let mut game = Game::new();
    game.add_players();
    game.initiate_game_loop();
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
    players: VecDeque<Player>,
    round_number: i8,
    // scoreboard
    settings: Settings,
}

impl Game {
    pub fn new() -> Game {
        Game {
            players: VecDeque::new(),
            round_number: 0,
            settings: Settings::new(),
        }
    }

    pub fn add_players(&mut self) {
        /*
        TODO: Make configurable number of players
        TODO: Configure prompts
        TODO: Allow "exit" command"
        */
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
        self.players.push_back(Player::new(name));
    }

    fn a_player_has_zero_chips(&self) -> bool {
        for player in &self.players {
            if player.chips == 0 {
                return true;
            }
        }
        false
    }

    pub fn initiate_game_loop(&mut self) {
        while self.round_number < self.settings.number_of_rounds || !self.a_player_has_zero_chips()
        {
            Round::run(&mut self.players);
        }
    }
}
