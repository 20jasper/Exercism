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
    dice.iter().fold([0; 6], |mut frequencies, value| {
        let index = (value - 1) as usize;
        frequencies[index] += 1;

        frequencies
    })
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
    let index = usize::from(value - 1);

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
        Category::Yacht => {
            // QUESTION is this a good way to handle the error if we know
            // `_dice` must contain 5 `u8`s?
            let first = _dice.first().expect("No dice in array!");
            // QUESTION why does `all` need a mutable iterator?
            if _dice.iter().all(|x| first == x) {
                50
            } else {
                0
            }
        }
        _ => todo!(),
    }
}
