mod dice;

use dice::*;

pub enum Category {
    Ones = 1,
    Twos = 2,
    Threes = 3,
    Fours = 4,
    Fives = 5,
    Sixes = 6,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

fn get_straight_score(offset: u8, dice: &Dice) -> u8 {
    let mut sorted_dice = *dice;
    sorted_dice.sort_unstable();

    let is_straight = sorted_dice
        .iter()
        .enumerate()
        .all(|(i, value)| *value == (i as u8 + offset));

    if is_straight {
        30
    } else {
        0
    }
}

// QUESTION why are `_dice` and `_category` prefixed with an underscore, that
// makes me think they are private or unused
pub fn score(_dice: Dice, _category: Category) -> u8 {
    match _category {
        Category::Ones
        | Category::Twos
        | Category::Threes
        | Category::Fours
        | Category::Fives
        | Category::Sixes => {
            let value = _category as u8;
            value * get_die_frequency(value, &_dice)
        }
        Category::FullHouse => {
            match (
                get_die_with_frequency(2, &_dice),
                get_die_with_frequency(3, &_dice),
            ) {
                (Some(two_dice), Some(three_dice)) => two_dice * 2 + three_dice * 3,
                _ => 0,
            }
        }
        Category::FourOfAKind => match get_die_with_at_least_frequency(4, &_dice) {
            Some(value) => 4 * value,
            None => 0,
        },
        Category::LittleStraight => get_straight_score(1, &_dice),
        Category::BigStraight => get_straight_score(2, &_dice),
        Category::Choice => _dice.iter().sum(),
        Category::Yacht => match get_die_with_frequency(5, &_dice) {
            Some(_) => 50,
            None => 0,
        },
    }
}
