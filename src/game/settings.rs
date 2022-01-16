use std::env;

pub struct Settings {
    // Default: 10 rounds
    pub number_of_rounds: i8,

    // Default: 0 points
    pub dec_ribbon_worth_a_point: bool,

    // Default: 0 additional points
    pub four_birds_is_ten_points: bool,

    // Default: may animal can only be an animal
    pub may_animal_can_be_double_junk: bool,

    // Default: winner gets an additional point
    pub no_winners_doubles_next_round: bool,

    // Default: 7+ animals has no significance
    pub seven_animals_doubles_win: bool,

    // Default: losing with zero animals has no significance
    pub zero_animals_doubles_loss: bool,

    // Default: losing with zero ribbons has no significance
    pub zero_ribbons_doubles_loss: bool,
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            number_of_rounds: if !env::var("NUMBER_OF_ROUNDS").is_err() {
                10
            } else {
                env::var("NUMBER_OF_ROUNDS")
                    .expect("a string")
                    .parse::<i8>()
                    .expect("Enter a number for env variable NUMBER_OF_ROUNDS")
            },
            dec_ribbon_worth_a_point: !env::var("DEC_RIBBON_WORTH_A_POINT").is_err(),
            four_birds_is_ten_points: !env::var("FOUR_BIRDS_IS_TEN_POINTS").is_err(),
            may_animal_can_be_double_junk: !env::var("MAY_ANIMAL_CAN_BE_DOUBLE_JUNK").is_err(),
            no_winners_doubles_next_round: !env::var("NO_WINNERS_DOUBLES_NEXT_ROUND").is_err(),
            seven_animals_doubles_win: !env::var("SEVEN_ANIMALS_DOUBLES_WIN").is_err(),
            zero_animals_doubles_loss: !env::var("ZERO_ANIMALS_DOUBLES_LOSS").is_err(),
            zero_ribbons_doubles_loss: !env::var("ZERO_RIBBONS_DOUBLES_LOSS").is_err(),
        }
    }
}
