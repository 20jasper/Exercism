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
