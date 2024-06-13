mod problems;

// use problems::two_sum::{self};

use problems::three_sum::{self};
fn main() {
    let nums = vec![1, 7, 9, 3, 4];
    let target = 30;
    let result = three_sum::three_sum(nums, target);
    println!("Result for Problem: {:?}", result);
}
