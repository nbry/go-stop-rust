use crate::active_cards::ActiveCards;
use crate::deck::Deck;
use crate::player::Player;

struct GameState {
    players: Vec<Player>,
    whose_turn: i32,
    deck: Deck,
    river: ActiveCards,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            players: Vec::new(),
            whose_turn: 0,
            deck: Deck::new(true),
            river: ActiveCards::new(),
        }
    }

    pub fn add_player(&mut self, name: String) {
        self.players.push(Player::new(name));
    }

    pub fn deal_cards() {}

    pub fn initiate_game_loop() {}
}

// Enter players
// Set junk card threshold
// Deal cards
// Start game loop
// - Player plays card
// **PLAY START**
// - if choice -> Player chooses card to match
// - else if match -> card added to match
// - else -> card added to river
// - Card is flipped
// - Check river for match
// - Check "match" for match -> if match, "sah"
// - if 4 cards match, if all the same suit, "tuk tuk"
// - if match, add match vector to player's captures
// - if river empty, "sak su di"

// - count points. If win threshold is met, ask player if go.
// - if go, <player>.go ++
// - else game end
