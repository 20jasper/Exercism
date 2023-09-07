// QUESTION is it worth the overhead of a hashmap to avoid needing these
// functions to convert between indices and die values?
// should I have just made a 7 long array to directly map to each number?
// is there another option I'm missing?

// QUESTION I see some solutions using a BTreeMap, what are some tradeoffs for
// using a BTreeMap vs a HashMap? Is it significant here, or is it just
// preference?

pub type Dice = [u8; 5];

fn get_dice_frequencies(dice: &Dice) -> [u8; 7] {
    // QUESTION why does `fold` need a mutable iterator?
    dice.iter()
        .fold([0; 7], |mut frequencies, value| {
            frequencies[*value as usize] += 1;

            frequencies
        })
}

pub fn get_die_frequency(value: u8, dice: &Dice) -> u8 {
    get_dice_frequencies(dice)[value as usize]
}

/// gets a die with a frequency returning true for a predicate
/// ```rust
/// let dice = [1, 1, 1, 2, 2];
/// assert_eq!(yacht::dice::get_die_with_frequency(|frequency| *frequency == 2, &dice), Some(2));
/// assert_eq!(yacht::dice::get_die_with_frequency(|frequency| *frequency > 4, &dice), None);
/// ```
pub fn get_die_with_frequency<Fn>(predicate: Fn, dice: &Dice) -> Option<u8>
where
    Fn: FnMut(&u8) -> bool,
{
    let frequencies = get_dice_frequencies(dice);

    let value = frequencies.iter().position(predicate)?;

    Some(value as u8)
}
