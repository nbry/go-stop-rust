mod card;
mod card_collection;
mod deck;
mod game;
mod hand;
mod player;

use card::Draw;
use deck::Deck;

fn main() {
    let mut the_deck = Deck::new(true);
    let mut collection = card_collection::CardCollection::new();
    collection.set_junk_animal_is_junk(true);

    for _i in 0..48 {
        let card = Draw::draw(&mut the_deck);
        collection.add_card(card);
    }

    collection.update_score();

    println!("{:#?}", collection);
    println!("{}", collection.points);
}
