use go_stop_rust::{active_cards, capture_pile, card, deck};

use card::{Card, Draw};
use deck::Deck;
use go_stop_rust::card::Suit;

#[test]
fn test_count_points_for_entire_deck() {
    let mut deck = Deck::new(true);
    let mut stack_of_cards: Vec<Card> = Vec::new();

    let mut captures = capture_pile::CapturePile::new();
    captures.set_sep_animal_is_junk(true);

    for _i in 0..48 {
        let card = Draw::draw(&mut deck);
        stack_of_cards.push(card);
    }

    captures.add_cards(stack_of_cards);

    assert_eq!(captures.points, 63);
}

#[test]
fn test_active_cards() {
    let mut the_deck = Deck::new(false);
    let mut stack_of_cards: Vec<Card> = Vec::new();
    let mut river = active_cards::ActiveCards::new();

    // Un-shuffled deck... so first 4 cards are December cards
    for _i in 0..4 {
        let card = Draw::draw(&mut the_deck);
        stack_of_cards.push(card);
    }

    river.add_cards(stack_of_cards);

    let cards_in_river_count = river
        .cards
        .get(&Suit::Dec)
        .expect("ard with suit Dec")
        .len();

    assert_eq!(cards_in_river_count, 4)
}
