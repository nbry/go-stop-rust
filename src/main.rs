mod card;
mod deck;
mod game;
mod hand;
mod player;
mod score;

use card::Draw;
use deck::Deck;

fn main() {
    let mut the_deck = Deck::new(true);
    let mut the_score = score::Score::new();

    for _i in 0..48 {
        let card = Draw::draw(&mut the_deck);
        the_score.add_card(card);
    }

    println!("{:#?}", the_score);
    println!("{}", the_score.get_total());
}
