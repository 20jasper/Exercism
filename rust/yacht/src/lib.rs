mod dice;

use dice::*;

pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

// QUESTION, is it possible to make a subset of an enum? Like if I wanted
// `category` to only allow `Ones` through `Sixes`
//
// QUESTION is there a better way to match enum variants to a value?
pub fn category_to_die_value(category: Category) -> Result<u8, &'static str> {
    let value = match category {
        Category::Ones => 1,
        Category::Twos => 2,
        Category::Threes => 3,
        Category::Fours => 4,
        Category::Fives => 5,
        Category::Sixes => 6,
        _ => return Err("Invalid Category variant passed, please pass Ones through Sixes"),
    };

    Ok(value)
}

/// Get score for categories like "Ones" and "Twos", where points are equal to
/// the value of the die * its frequency
///
/// Example
///
/// let category = Category::Fives;
/// let dice = [5_u8; 5];
///
/// let actual = get_upper_section_score(category, &dice);
/// assert_eq!(actual, 25);
fn get_upper_section_score(category: Category, dice: &Dice) -> Result<u8, &'static str> {
    let value = category_to_die_value(category)?;

    Ok(value * get_die_frequency(value, dice))
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
            // QUESTION is this appropriate error handling if this should never fail?
            get_upper_section_score(_category, &_dice).expect("failed to get score")
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
