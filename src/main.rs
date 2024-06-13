mod problems;

use problems::two_sum::{self};

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = two_sum::two_sum(nums, target);
    println!("Result for Problem 1 (two_sum): {:?}", result);
}
