use crate::game::cards::{active::ActiveCards, deck::Deck};

use super::player::Players;

#[derive(Debug)]
struct Round {
    deck: Deck,
    players: Players,
    river: ActiveCards,
}

pub fn run_round(players: Players) {
    println!("ROUND STARTED");

    let mut pre_play = Round {
        deck: Deck::new(true),
        players,
        river: ActiveCards::new(),
    };

    //TODO: Implement cut and tap
    pre_play
        .players
        .deal_cards(false, &mut pre_play.deck, &mut pre_play.river);

    println!("RIVER: {:#?}", pre_play.river);
    println!("{:#?}", pre_play.players);

    // pre_play.determine_active_players()
}
