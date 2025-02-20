use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::collections::HashSet;

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

/// LeetCode problem 217.
/// Determines whether array has duplicates.
/// O(n) time complexity
/// O(n) space complexity
///
/// # Arguments
///
/// * `nums` - A vector of integers.
///
/// # Returns
/// True if any value appears at least twice in the array, false if every element is distinct.
///
/// # Examples
/// ```
/// use std::collections::HashSet;
/// use leetcode_rust::problems::arrays_hashing::has_duplicate;
/// let nums = vec![1, 2, 3, 4, 5];
/// assert_eq!(has_duplicate(nums), false);
/// ```
///
/// ```
/// use std::collections::HashSet;
/// use leetcode_rust::problems::arrays_hashing::has_duplicate;
/// let nums = vec![1, 2, 3, 3, 5];
/// assert_eq!(has_duplicate(nums), true);
/// ```
pub fn has_duplicate(nums: Vec<i32>) -> bool {
    let mut seen: HashSet<i32> = HashSet::new();

    for i in 0..nums.len() {
        if seen.contains(&nums[i]) {
            return true;
        } else {
            seen.insert(nums[i]);
        }
    }
    return false;
}

/// LeetCode problem 242.
/// Determines if two strings are anagrams of each other.
/// O(n) time complexity
/// O(n) space complexity
///
/// # Arguments
///
/// * `s` - A string.
/// * `t` - A string.
///
/// # Returns
/// True if `s` and `t` are anagrams, false otherwise.
///
/// # Examples
/// ```
/// use leetcode_rust::problems::arrays_hashing::is_anagram;
/// let s = "anagram".to_string();
/// let t = "nagaram".to_string();
/// assert_eq!(is_anagram(s, t), true);
/// ```
///
/// ```
/// use leetcode_rust::problems::arrays_hashing::is_anagram;
/// let s = "rat".to_string();
/// let t = "car".to_string();
/// assert_eq!(is_anagram(s, t), false);
/// ```
pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut counts = HashMap::new();

    for ch in s.chars() {
        *counts.entry(ch).or_insert(0) += 1;
    }

    for ch in t.chars() {
        let count = counts.entry(ch).or_insert(0);
        if *count == 0 {
            return false;
        }
        *count -= 1;
    }

    counts.values().all(|&count| count == 0)
}

/// LeetCode problem 125.
/// Checks if a given string is a palindrome, considering only alphanumeric characters
/// and ignoring case.
/// 
/// This function runs in **O(n)** time complexity and **O(n)** space complexity.
/// 
/// # Arguments
/// 
/// * `s` - A `String` containing the input text.
/// 
/// # Returns
/// 
/// Returns `true` if `s` is a palindrome, otherwise returns `false`.
/// 
/// # Examples
/// 
/// ```
/// use leetcode_rust::problems::arrays_hashing::is_palindrome;
/// 
/// let s = "A man, a plan, a canal: Panama".to_string();
/// assert_eq!(is_palindrome(s), true);
/// ```
///
/// ```
/// use leetcode_rust::problems::arrays_hashing::is_palindrome;
/// 
/// let s = "race a car".to_string();
/// assert_eq!(is_palindrome(s), false);
/// ```
///
/// ```
/// use leetcode_rust::problems::arrays_hashing::is_palindrome;
/// 
/// let s = " ".to_string(); // Empty or whitespace-only strings are palindromes.
/// assert_eq!(is_palindrome(s), true);
/// ```
pub fn is_palindrome(s: String) -> bool {
    
    let mut s_alphanums = String::new();
    for c in s.chars() {
        if c.is_alphanumeric(){
            s_alphanums.push(c.to_ascii_lowercase());
        }
    }
    let mut s_reverse = String::new();
    for c in s_alphanums.chars().rev() {
        s_reverse.push(c);
    }

    return s_alphanums == s_reverse;
}

/// LeetCode problem 704.
/// Implements binary search to find the target value in a sorted array.
/// 
/// This function runs in **O(log n)** time complexity and **O(1)** space complexity.
/// 
/// # Arguments
/// 
/// * `nums` - A `Vec<i32>` representing a sorted list of integers.
/// * `target` - An `i32` representing the value to search for.
/// 
/// # Returns
/// 
/// Returns the index of `target` if found in `nums`, otherwise returns `-1`.
/// 
/// # Examples
/// 
/// ```
/// use leetcode_rust::problems::binary_search::search;
/// 
/// let nums = vec![-1, 0, 3, 5, 9, 12];
/// let target = 9;
/// assert_eq!(search(nums, target), 4);
/// ```
///
/// ```
/// use leetcode_rust::problems::binary_search::search;
/// 
/// let nums = vec![-1, 0, 3, 5, 9, 12];
/// let target = 2;
/// assert_eq!(search(nums, target), -1);
/// ```
///
/// ```
/// use leetcode_rust::problems::binary_search::search;
/// 
/// let nums = vec![5];
/// let target = 5;
/// assert_eq!(search(nums, target), 0);
/// ```
///
///
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left: i32 = 0;
    let mut right: i32 = nums.len() as i32 - 1; // Use i32 to avoid underflow issues

    while left <= right {
        let middle = left + (right - left) / 2; // Safe midpoint calculation

        if nums[middle as usize] == target {
            return middle;
        } else if nums[middle as usize] < target {
            left = middle + 1;
        } else {
            right = middle - 1;
        }
    }
    -1
}
