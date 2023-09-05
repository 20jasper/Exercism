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

// how can I make this more generic? Is there a way to make `possible_matches`
// accept anything that can be turned into an iterator of T?
fn get_frequency<T>(target: T, possible_matches: &[T]) -> u8
where
    T: Eq,
{
    possible_matches
        .iter()
        .fold(0, |frequency, possible_match| {
            if *possible_match == target {
                frequency + 1
            } else {
                frequency
            }
        })
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
fn get_upper_section_score(category: Category, dice: &Dice) -> u8 {
    // QUESTION, is it possible to make a subset of an enum? Like if I wanted
    // `category` to only allow `Ones` through `Sixes`
    //
    // QUESTION is there a better way to match enum variants to a value?
    let value = match category {
        Category::Ones => 1,
        Category::Twos => 2,
        Category::Threes => 3,
        Category::Fours => 4,
        Category::Fives => 5,
        Category::Sixes => 6,
        _ => panic!("Invalid Category variant passed, please pass Ones through Sixes"),
    };
    value * get_frequency(value, dice)
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
        | Category::Sixes => get_upper_section_score(_category, &_dice),
        Category::Yacht => {
            // why does `all` need a mutable iterator?
            let mut iter = _dice.iter();
            let first = _dice.first().unwrap();
            if iter.all(|x| first == x) {
                50
            } else {
                0
            }
        }
        _ => todo!(),
    }
}
