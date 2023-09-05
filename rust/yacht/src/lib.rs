// QUESTION is it worth the overhead of a hashmap to avoid needing these
// functions to convert between indices and die values?
// should I have just made a 7 long array to directly map to each number?
// is there another option I'm missing?
const INDEX_TO_DIE_VALUE_OFFSET: u8 = 1;

fn index_to_die_value(index: usize) -> u8 {
    index as u8 + INDEX_TO_DIE_VALUE_OFFSET
}

fn die_value_to_index(die_value: u8) -> usize {
    (die_value - INDEX_TO_DIE_VALUE_OFFSET) as usize
}

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

fn get_dice_frequencies(dice: &Dice) -> [u8; 6] {
    // QUESTION why does `fold` need a mutable iterator?
    dice.iter().fold([0; 6], |mut frequencies, value| {
        let index = die_value_to_index(*value);
        frequencies[index] += 1;

        frequencies
    })
}

// QUESTION is there a good way to generalize this function so that it can be
// used to get dice with more than or equal to the frequency passed? I thought
// of passing a closure, but wasn't sure if it was a good idea
/// gets a die with an exact frequency
/// let dice = [1, 1, 1, 2, 2];
/// assert_eq!(get_die_with_frequency(2, &dice), Some(2));
/// assert_eq!(get_die_with_frequency(5, &dice), None);
fn get_die_with_frequency(target: u8, dice: &Dice) -> Option<u8> {
    let frequencies = get_dice_frequencies(dice);

    let (index, _) = frequencies
        .iter()
        .enumerate()
        .find(|(_, frequency)| **frequency == target)?;

    Some(index_to_die_value(index))
}

// QUESTION, is it possible to make a subset of an enum? Like if I wanted
// `category` to only allow `Ones` through `Sixes`
//
// QUESTION is there a better way to match enum variants to a value?
fn category_to_die_value(category: Category) -> Result<u8, &'static str> {
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

fn get_die_frequency(value: u8, dice: &Dice) -> u8 {
    let index = die_value_to_index(value);

    get_dice_frequencies(dice)[index]
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

type Dice = [u8; 5];
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
        Category::Choice => _dice.iter().sum(),
        Category::Yacht => match get_die_with_frequency(5, &_dice) {
            Some(_) => 50,
            None => 0,
        },
        _ => todo!(),
    }
}
