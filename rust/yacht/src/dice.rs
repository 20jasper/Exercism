// QUESTION is it worth the overhead of a hashmap to avoid needing these
// functions to convert between indices and die values?
// should I have just made a 7 long array to directly map to each number?
// is there another option I'm missing?

use std::collections::HashMap;

pub type Dice = [u8; 5];

fn get_dice_frequencies(dice: &Dice) -> HashMap<u8, u8> {
    // QUESTION why does `fold` need a mutable iterator?
    dice.iter()
        .fold(HashMap::with_capacity(6), |mut frequencies, value| {
            frequencies
                .entry(*value)
                .and_modify(|x| *x += 1)
                .or_insert(1);

            frequencies
        })
}

pub fn get_die_frequency(value: u8, dice: &Dice) -> u8 {
    *get_dice_frequencies(dice)
        .get(&value)
        .unwrap_or(&0)
}

// QUESTION is there a good way to generalize this function so that it can be
// used to get dice with more than or equal to the frequency passed (like in
// `get_die_with_at_least_frequency`)? I thought of passing a closure, but
// wasn't sure if it was a good idea
/// gets a die with an exact frequency
/// let dice = [1, 1, 1, 2, 2];
/// assert_eq!(get_die_with_frequency(2, &dice), Some(2));
/// assert_eq!(get_die_with_frequency(5, &dice), None);
pub fn get_die_with_frequency(target: u8, dice: &Dice) -> Option<u8> {
    let frequencies = get_dice_frequencies(dice);

    let (value, _) = frequencies
        .iter()
        .find(|(_, frequency)| **frequency == target)?;

    Some(*value)
}

pub fn get_die_with_at_least_frequency(target: u8, dice: &Dice) -> Option<u8> {
    let frequencies = get_dice_frequencies(dice);

    let (value, _) = frequencies
        .iter()
        .find(|(_, frequency)| **frequency >= target)?;

    Some(*value)
}
