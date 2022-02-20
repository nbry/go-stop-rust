pub mod cards;
mod player;
mod round;
mod score;
mod settings;

use player::Players;
use settings::Settings;

use self::round::run_pre_play;

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
        let mut round = run_pre_play(game.players);
        game.players = round.run_round();
    }
}
