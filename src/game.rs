pub mod cards;
mod player;
mod round;
mod score;
mod settings;

use self::round::run_round;
use player::Players;
use settings::Settings;
use std::io;

/// TODO:
/// - Last player cuts deck
/// - Last player peeks bottom card
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

struct Game {
    players: Players,
    round_number: i8,
    settings: Settings,
}

pub fn run_game_loop() {
    let mut game = Game {
        players: Players::new(),
        round_number: 0,
        settings: Settings::new(),
    };

    game.players.add_players();

    while game.round_number < game.settings.number_of_rounds
        || !game.players.a_player_has_zero_chips()
    {
        run_round(game.players);
        game.players = Players::new();

        let mut pause = String::new();
        println!("EXECUTION PAUSED FOR DEBUGGING");
        io::stdin().read_line(&mut pause).expect("a string");
    }
}
