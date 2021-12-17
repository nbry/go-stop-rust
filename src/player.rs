use crate::card_collection::CardCollection;

pub struct Player {
    name: String,
    go: i32,
    chips: i32,
    collection: CardCollection,
}

impl Player {
    fn new(name: String) -> Player {
        Player {
            name,
            go: 0,
            chips: 0,
            collection: CardCollection::new(),
        }
    }

    fn get_score(&mut self) {
        self.collection.set_junk_animal_is_junk(true);
    }
}
