pub mod player;
mod round;
mod score;
pub mod settings;

use player::Players;
use round::Round;
use settings::Settings;

pub fn run() {
    let mut game = Game::new();
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
    players: Players,
    round_number: i8,
    settings: Settings,
}

impl Game {
    pub fn new() -> Game {
        let mut new_game = Game {
            players: Players::new(),
            round_number: 0,
            settings: Settings::new(),
        };

        new_game.players.add_players();

        new_game
    }

    pub fn initiate_game_loop(&mut self) {
        while self.round_number < self.settings.number_of_rounds
            || !self.players.a_player_has_zero_chips()
        {
            Round::run(&mut self.players);
        }
    }
}
