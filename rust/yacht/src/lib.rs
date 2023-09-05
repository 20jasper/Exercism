pub mod category;
mod dice;

use category::*;
use dice::*;

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
        Category::FourOfAKind => {
            let frequencies = get_dice_frequencies(&_dice);

            let four_of_a_kind = frequencies
                .iter()
                .enumerate()
                .find(|(_, frequency)| **frequency >= 4);

            match four_of_a_kind {
                Some((i, _)) => 4 * index_to_die_value(i),
                None => 0,
            }
        }
        Category::LittleStraight => get_straight_score(1, &_dice),
        Category::BigStraight => get_straight_score(2, &_dice),
        Category::Choice => _dice.iter().sum(),
        Category::Yacht => match get_die_with_frequency(5, &_dice) {
            Some(_) => 50,
            None => 0,
        },
    }
}
