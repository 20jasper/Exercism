pub fn square(s: u32) -> u64 {
    match s {
        1..=64 => 2_u64.pow(s - 1),
        _ => panic!("Square must be between 1 and 64"),
    }
}

pub fn total() -> u64 {
    // solution 1: return the answer
    // 18_446_744_073_709_551_615
    // u64::MAX

    // solution 2: for loop
    // let mut total = 0u64;
    // for i in 1..=64 {
    //     total += square(i)
    // }
    // total

    // solution 3: reduce
    // (1..=64)
    //     .reduce(|acc: u64, i| acc + square(i as u32))
    //     .unwrap()
}
