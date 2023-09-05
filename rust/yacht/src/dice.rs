// QUESTION is it worth the overhead of a hashmap to avoid needing these
// functions to convert between indices and die values?
// should I have just made a 7 long array to directly map to each number?
// is there another option I'm missing?
const INDEX_TO_DIE_VALUE_OFFSET: u8 = 1;

pub fn index_to_die_value(index: usize) -> u8 {
    index as u8 + INDEX_TO_DIE_VALUE_OFFSET
}

fn die_value_to_index(die_value: u8) -> usize {
    (die_value - INDEX_TO_DIE_VALUE_OFFSET) as usize
}

pub type Dice = [u8; 5];

pub fn get_dice_frequencies(dice: &Dice) -> [u8; 6] {
    // QUESTION why does `fold` need a mutable iterator?
    dice.iter()
        .fold([0; 6], |mut frequencies, value| {
            let index = die_value_to_index(*value);
            frequencies[index] += 1;

            frequencies
        })
}

pub fn get_die_frequency(value: u8, dice: &Dice) -> u8 {
    let index = die_value_to_index(value);

    get_dice_frequencies(dice)[index]
}

// QUESTION is there a good way to generalize this function so that it can be
// used to get dice with more than or equal to the frequency passed? I thought
// of passing a closure, but wasn't sure if it was a good idea
/// gets a die with an exact frequency
/// let dice = [1, 1, 1, 2, 2];
/// assert_eq!(get_die_with_frequency(2, &dice), Some(2));
/// assert_eq!(get_die_with_frequency(5, &dice), None);
pub fn get_die_with_frequency(target: u8, dice: &Dice) -> Option<u8> {
    let frequencies = get_dice_frequencies(dice);

    let (index, _) = frequencies
        .iter()
        .enumerate()
        .find(|(_, frequency)| **frequency == target)?;

    Some(index_to_die_value(index))
}
