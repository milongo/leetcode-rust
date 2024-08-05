use std::cmp::max;
use std::cmp::min;
use std::collections::HashSet;
use std::collections::HashMap;


/// LeetCode problem 121.
/// Returns the maximum profit one can obtain by buying and selling stock.
/// O(n) time complexity.
/// O(1) space complexity.
///
/// # Arguments
///
/// * `prices` - A vector of integers.
///
/// # Returns
///
/// The maximum profit possible from buying and selling stock.
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_profit = 0;
    let mut min_price = prices[0];

    for i in 0..prices.len() {
        if prices[i] > min_price {
            max_profit = max(max_profit, prices[i] - min_price)
        } else {
            min_price = min(min_price, prices[i]);
        }
    }
    return max_profit;
}



/// LeetCode problem 3.
/// Computes the length of the longest substring without repeated characters.
/// O(n) time complexity.
/// O(n) space complexity.
///
/// # Arguments
///
/// * `s` - A string.
///
/// # Returns
///
/// The length of the longest substring without repeated characters.
/// # Examples
/// ```
/// use std::assert_eq;
/// use leetcode_rust::problems::arrays_hashing::length_of_longest_substring;
/// let string = String::from("abcdabca");
/// assert_eq!(length_of_longest_substring(string), 4);
/// ```
///
/// # Notes
///
/// The function is case-sensitive, meaning that 'A' and 'a' are considered different characters.
///
/// ```
/// use std::assert_eq;
/// use leetcode_rust::problems::arrays_hashing::length_of_longest_substring;
/// let string = String::from("abcABC");
/// assert_eq!(length_of_longest_substring(string), 6);
/// ```
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut longest_substring: i32 = 0;
    let mut start = 0;
    let mut seen: HashSet<char> = HashSet::new();
    let s_chars: Vec<char> = s.chars().collect();

    for stop in 0..s.len() {
        while seen.contains(&s_chars[stop]) {
            seen.remove(&s_chars[start]);
            start += 1;
        }

        let window_length: i32 = ((stop - start) as i32) + 1;
        longest_substring = max(window_length, longest_substring);
        seen.insert(s_chars[stop]);
    }
    longest_substring
}



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
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen = HashMap::new();

    for i in 0..nums.len() {
        let num = nums[i];
        let complement = target - num;
        if seen.contains_key(&complement) {
            return vec![seen[&complement] as i32, i as i32];
        } else {
            seen.insert(num, i);
        }
    }
    return vec![];
}

/// Variant of LeetCode problem 15.
/// Determines whether any three numbers in an array can be added to a target number.
/// O(n**2) time complexity.
///
/// # Arguments
/// * `nums` - A vector of integers.
/// * `target` - The target sum.
///
/// # Returns
///
/// True if target can be obtained by adding three numbers in the array, false otherwise.
pub fn three_sum_bool(nums: Vec<i32>, target: i32) -> bool {
    let mut nums = nums;
    nums.sort();

    let n = nums.len();

    for i in 0..n - 2 {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        let mut left = i + 1;
        let mut right = n - 1;

        while left < right {
            let sum = nums[i] + nums[left] + nums[right];

            if sum == target {
                return true;
            } else if sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }

    return false;
}
