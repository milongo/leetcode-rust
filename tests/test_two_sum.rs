use leetcode_rust::problems::two_sum;

#[test]
fn test_two_sum_example() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let expected = vec![0, 1];
    let result = two_sum::two_sum(nums, target);
    assert_eq!(result, expected);
}

#[test]
fn test_two_sum_empty() {
    let nums = vec![];
    let target = 9;
    let expected = vec![];
    let result = two_sum::two_sum(nums, target);
    assert_eq!(result, expected);
}
