pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let k = k as usize;
    let sum = nums.iter().take(k).sum::<i32>();

    let max = std::iter::once(sum)
        .chain(
            nums
                // create iterator holding tuples of left and right of window
                .iter()
                .zip(nums.iter().skip(k))
                // store sum at each step
                .scan(sum, |state, (left, right)| {
                    *state -= left;
                    *state += right;
                    Some(*state)
                }),
        )
        // add original state
        .max()
        .unwrap();

    max as f64 / k as f64
}
