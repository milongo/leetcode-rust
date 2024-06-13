use rust::problems::three_sum::{self, three_sum};

#[test]
fn test_three_sum_example_1() {
    let nums = vec![3,1 , 4, 7, 9];
    let target = 8;
    let expected = true;
    let result = three_sum::three_sum(nums, target);
    assert_eq!(result, expected);
}

#[test]
fn test_three_sum_example_2() {
    let nums = vec![20, 1, 3, 4, 8];
    let target = 13;
    let expected = true;
    let result = three_sum::three_sum(nums, target);
    assert_eq!(result, expected);
}
