use std::collections::HashMap;

/// LeetCode Problem 1.
/// Finds two numbers in the given list that add up to the target value.
/// O(n) time complexity.
/// O(n) space complexity.
/// 
/// # Arguments
/// 
/// * `nums` - A vector of integers.
/// * `target` - The target sum.
/// 
/// # Returns
/// 
/// A vector containing the indices of the two numbers that add up to the target value.
/// 
/// # Examples
/// 
/// ```
/// let result = two_sum(vec![2, 7, 11, 15], 9);
/// assert_eq!(result, vec![0, 1]);
/// ```
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        
    let mut seen = HashMap::new();
    
    for i in 0..nums.len() {
        let num = nums[i];
        let complement = target - num;
        if seen.contains_key(&complement) {
            return vec![seen[&complement] as i32, i as i32];
        }
        else {
            seen.insert(num, i);
        }
    }
    return vec![]
}
