pub mod cards;
mod player;
mod round;
mod score;
mod settings;

use self::round::run_round;
use player::Player;
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

/// Singleton struct for a game of Go-Stop
struct Game {
    players: Vec<Player>,
    round_number: i8,
    settings: Settings,
}

pub fn run_game_loop() {
    let mut game = Game {
        players: Vec::new(),
        round_number: 0,
        settings: Settings::new(),
    };

    add_players(&mut game.players);

    while game.round_number < game.settings.number_of_rounds
        || !a_player_has_zero_chips(&game.players)
    {
        game.players = run_round(game.players);

        let mut pause = String::new();
        println!("EXECUTION PAUSED FOR DEBUGGING");
        io::stdin().read_line(&mut pause).expect("a string");
    }
}

pub fn add_players(players: &mut Vec<Player>) {
    //TODO: Implement 4-5 player games
    const MIN_PLAYERS: usize = 2;
    const MAX_PLAYERS: usize = 3;
    println!("Enter 2-3 Players");

    while players.len() < MAX_PLAYERS {
        let mut entry = String::new();

        println!("Enter name of Player {}", players.len() + 1);
        io::stdin().read_line(&mut entry).expect("a string");

        if entry.trim().eq("Done") {
            if players.len() < MIN_PLAYERS {
                println!("Need at least 2 players");
            } else {
                break;
            }
        } else {
            add_player(players, entry.trim().to_string());
        }
    }
}

pub fn add_player(players: &mut Vec<Player>, mut name: String) {
    if name.eq("") {
        name = format!("Player {}", players.len() + 1);
    } else {
        let mut duplicate_name_count = 0;

        for player in players.iter_mut() {
            if name.eq(&player.name) || name.eq(&player.name[0..player.name.len() - 3]) {
                duplicate_name_count += 1;
            }
        }

        if duplicate_name_count > 0 {
            name = format!("{}({})", name, duplicate_name_count);
        }
    }

    println!("Adding {}!", name);
    players.push(Player::new(name, players.len()));
}

pub fn a_player_has_zero_chips(players: &Vec<Player>) -> bool {
    for player in players {
        if player.chips == 0 {
            return true;
        }
    }
    false
}
