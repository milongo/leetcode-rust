use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

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
        if c.is_alphanumeric() {
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
/// use leetcode_rust::problems::arrays_hashing::search;
///
/// let nums = vec![-1, 0, 3, 5, 9, 12];
/// let target = 9;
/// assert_eq!(search(nums, target), 4);
/// ```
///
/// ```
/// use leetcode_rust::problems::arrays_hashing::search;
///
/// let nums = vec![-1, 0, 3, 5, 9, 12];
/// let target = 2;
/// assert_eq!(search(nums, target), -1);
/// ```
///
/// ```
/// use leetcode_rust::problems::arrays_hashing::search;
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

/// LeetCode problem 20.
/// Determines if a string of parentheses, brackets, and braces is valid.
///
/// A string is considered **valid** if:
/// - Open brackets must be closed by the same type of brackets.
/// - Open brackets must be closed in the correct order.
/// - Every close bracket has a corresponding open bracket of the same type.
///
/// # Arguments
///
/// * `s` - A `String` containing only the characters `()[]{}`.
///
/// # Returns
///
/// Returns `true` if `s` is a valid sequence of brackets, otherwise returns `false`.
///
/// # Complexity
///
/// - **Time Complexity:** O(n) – Each character is processed once.
/// - **Space Complexity:** O(n) – In the worst case, the stack stores all characters.
///
/// # Examples
///
/// ```
/// use leetcode_rust::problems::stack::is_valid;
///
/// let s = "()".to_string();
/// assert_eq!(is_valid(s), true);
/// ```
///
/// ```
/// use leetcode_rust::problems::stack::is_valid;
///
/// let s = "()[]{}".to_string();
/// assert_eq!(is_valid(s), true);
/// ```
///
/// ```
/// use leetcode_rust::problems::stack::is_valid;
///
/// let s = "(]".to_string();
/// assert_eq!(is_valid(s), false);
/// ```
///
/// ```
/// use leetcode_rust::problems::stack::is_valid;
///
/// let s = "([)]".to_string();
/// assert_eq!(is_valid(s), false);
/// ```
///
/// ```
/// use leetcode_rust::problems::stack::is_valid;
///
/// let s = "{[]}".to_string();
/// assert_eq!(is_valid(s), true);
/// ```
pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();
    let mut map = HashMap::new();
    map.insert(')', '(');
    map.insert(']', '[');
    map.insert('}', '{');

    for char in s.chars() {
        if map.contains_key(&char) {
            if stack.len() > 0 && stack[stack.len() - 1] == map[&char] {
                stack.pop();
            } else {
                return false;
            }
        } else {
            stack.push(char);
        }
    }
    return stack.len() == 0;
}

/// LeetCode problem 733.
/// Performs a flood fill on a 2D grid, changing all connected pixels of the same color to a new color.
///
/// This function implements an **iterative DFS (Depth-First Search)** using a stack to avoid recursion depth issues.
/// It modifies all adjacent pixels that share the same color as the starting pixel.
///
/// # Arguments
///
/// * `image` - A 2D vector (`Vec<Vec<i32>>`) representing the grid of pixel colors.
/// * `sr` - The row index of the starting pixel.
/// * `sc` - The column index of the starting pixel.
/// * `color` - The new color to apply to the connected region.
///
/// # Returns
///
/// A modified 2D vector (`Vec<Vec<i32>>`) where all pixels connected to `(sr, sc)` with the same original color
/// have been changed to `color`.
///
/// # Constraints
///
/// - The image dimensions are `m x n`, where `1 <= m, n <= 50`.
/// - The color values are non-negative integers.
/// - The starting pixel `(sr, sc)` is always within the image bounds.
///
/// # Complexity
///
/// - **Time Complexity:** O(m × n) in the worst case, where all pixels are the same color and need to be changed.
/// - **Space Complexity:** O(m × n) in the worst case due to the stack storing all pixels.
///
/// # Examples
///
/// ```
/// use leetcode_rust::problems::arrays_hashing::flood_fill;
///
/// let image = vec![
///     vec![1, 1, 1],
///     vec![1, 1, 0],
///     vec![1, 0, 1]
/// ];
/// let sr = 1;
/// let sc = 1;
/// let new_color = 2;
/// let expected = vec![
///     vec![2, 2, 2],
///
pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
    let starting_color = image[sr as usize][sc as usize];
    if color == starting_color {
        return image;
    }
    let mut coord_stack = Vec::new();
    coord_stack.push((sr, sc));
    let mut new_image = image.clone();
    let rows = image.len() as i32;
    let cols = image[0].len() as i32;
    while coord_stack.len() > 0 {
        if let Some(coords) = coord_stack.pop() {
            let x = coords.0;
            let y = coords.1;
            if new_image[x as usize][y as usize] == starting_color {
                new_image[x as usize][y as usize] = color;
            }
            if x + 1 <= rows - 1 {
                if new_image[x as usize + 1][y as usize] == starting_color {
                    coord_stack.push((x + 1, y))
                }
            }
            if x - 1 >= 0 {
                if new_image[x as usize - 1][y as usize] == starting_color {
                    coord_stack.push((x - 1, y))
                }
            }
            if y + 1 <= cols - 1 {
                if new_image[x as usize][y as usize + 1] == starting_color {
                    coord_stack.push((x, y + 1))
                }
            }
            if y - 1 >= 0 {
                if new_image[x as usize][y as usize - 1] == starting_color {
                    coord_stack.push((x, y - 1))
                }
            }
        }
    }
    new_image
}

/// LeetCode problem 383
/// Determines whether a ransom note can be constructed using letters from a magazine.
///
/// This function checks if the `ransom_note` string can be formed using the characters
/// available in `magazine`. Each character in `magazine` can only be used once.
///
/// # Arguments
///
/// * `ransom_note` - A `String` containing the text of the ransom note.
/// * `magazine` - A `String` containing the available characters.
///
/// # Returns
///
/// Returns `true` if `ransom_note` can be constructed using characters from `magazine`,
/// otherwise returns `false`.
///
/// # Complexity
///
/// - **Time Complexity:** O(n + m), where `n` is the length of `ransom_note` and `m` is the length of `magazine`.
/// - **Space Complexity:** O(m), since we store character counts from `magazine` in a `HashMap`.
///
/// # Examples
///
/// ```
/// use leetcode_rust::problems::hashmap::can_construct;
///
/// let ransom_note = "a".to_string();
/// let magazine = "b".to_string();
/// assert_eq!(can_construct(ransom_note, magazine), false);
/// ```
///
/// ```
/// use leetcode_rust::problems::hashmap::can_construct;
///
/// let ransom_note = "aa".to_string();
/// let magazine = "ab".to_string();
/// assert_eq!(can_construct(ransom_note, magazine), false);
/// ```
///
/// ```
/// use leetcode_rust::problems::hashmap::can_construct;
///
/// let ransom_note = "aa".to_string();
/// let magazine = "aab".to_string();
/// assert_eq!(can_construct(ransom_note, magazine), true);
/// ```
pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut mag_count: HashMap<char, i32> = HashMap::new();
    for char in magazine.chars() {
        *mag_count.entry(char).or_default() += 1;
    }

    for char in ransom_note.chars() {
        let count = mag_count.entry(char).or_default();
        if *count == 0 {
            return false;
        }
        *count -= 1;
    }
    true
}

/// LeetCode problem 409.
/// Computes the length of the longest palindrome that can be built using the characters in `s`.
///
/// A palindrome is a word or phrase that reads the same forward and backward.
/// This function determines the maximum possible length of a palindrome that can
/// be formed using the letter frequencies in `s`.
///
/// # Arguments
///
/// * `s` - A `String` containing lowercase and/or uppercase letters.
///
/// # Returns
///
/// Returns an `i32` representing the longest possible palindrome length.
///
/// # Approach
///
/// - Count the frequency of each character using a `HashMap<char, i32>`.
/// - Add **all even frequencies** to the palindrome length.
/// - Add **largest possible odd frequencies** by subtracting 1 (since one letter remains unused).
/// - If there's at least one odd-frequency character, **one odd character can be placed in the center**.
///
/// # Complexity
///
/// - **Time Complexity:** O(n), where `n` is the length of `s`, since we iterate over `s` and the frequency map once.
/// - **Space Complexity:** O(1), since the `HashMap` holds at most 52 entries (for lowercase + uppercase letters).
///
/// # Examples
///
/// ```
/// use leetcode_rust::problems::hashmap::longest_palindrome;
///
/// let s = "abccccdd".to_string();
/// assert_eq!(longest_palindrome(s), 7); // "dccaccd" is the longest palindrome
/// ```
///
/// ```
/// use leetcode_rust::problems::hashmap::longest_palindrome;
///
/// let s = "a".to_string();
/// assert_eq!(longest_palindrome(s), 1); // "a" is already a palindrome
/// ```
///
/// ```
/// use leetcode_rust::problems::hashmap::longest_palindrome;
///
/// let s = "bananas".to_string();
/// assert_eq!(longest_palindrome(s), 5); // "nanan" or "anana"
/// ```
pub fn longest_palindrome(s: String) -> i32 {
    let mut char_count = HashMap::new();
    for char in s.chars() {
        *char_count.entry(char).or_default() += 1;
    }

    let mut longest_length = 0;
    let mut has_odd_freq = false;
    for (key, val) in char_count.iter() {
        if val % 2 == 0 {
            longest_length += 2;
        } else {
            has_odd_freq = true;
            longest_length += val - 1;
        }
    }
    if has_odd_freq {
        longest_length += 1;
    }
    longest_length
}

/// LeetCode problem 169.
/// Finds the majority element in a vector using the **Boyer-Moore Voting Algorithm**.
///
/// The majority element is the element that appears more than `⌊n / 2⌋` times in the array.
/// This function assumes that such an element **always exists**, as per the problem constraints.
///
/// # Arguments
///
/// * `nums` - A vector of integers (`Vec<i32>`) containing the input numbers.
///
/// # Returns
///
/// Returns the integer that is the majority element.
///
/// # Approach
///
/// - Uses the **Boyer-Moore Voting Algorithm**, which efficiently finds the majority element in O(n) time.
/// - Maintains a `candidate` and a `count`.
/// - If `count` reaches zero, it updates `candidate` to the current element.
/// - If the current element matches `candidate`, it increments `count`, otherwise it decrements it.
/// - Since the majority element always exists, `candidate` will hold the correct answer at the end.
///
/// # Complexity
///
/// - **Time Complexity:** O(n), where `n` is the length of `nums`, since we traverse the array once.
/// - **Space Complexity:** O(1), as the algorithm uses only two integer variables.
///
/// # Examples
///
/// ```
/// use leetcode_rust::problems::arrays_hashing::majority_element;
///
/// let nums = vec![3, 2, 3];
/// assert_eq!(majority_element(nums), 3);
/// ```
///
/// ```
/// use leetcode_rust::problems::arrays_hashing::majority_element;
///
/// let nums = vec![2, 2, 1, 1, 1, 2, 2];
/// assert_eq!(majority_element(nums), 2);
/// ```
pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut candidate = nums[0];
    let mut count = 0;

    for num in &nums {
        if count == 0 {
            candidate = *num;
        }
        if candidate == *num {
            count += 1;
        } else {
            count -= 1;
        }
    }

    candidate
}

/// LeetCode problem 70.
/// Computes the number of distinct ways to climb `n` stairs.
///
/// Each time, you can either climb **1 step** or **2 steps**. This function returns
/// the total number of unique ways to reach the top, following the Fibonacci sequence.
///
/// # Arguments
///
/// * `n` - The total number of stairs to climb.
///
/// # Returns
///
/// Returns an `i32` representing the number of distinct ways to climb `n` stairs.
///
/// # Approach
///
/// This problem follows the **Fibonacci sequence**, where:
///
/// ```text
/// f(n) = f(n-1) + f(n-2)
/// ```
///
/// Instead of using recursion or storing all values in a `Vec`, we optimize by keeping
/// only the **last two computed values**, reducing space complexity to **O(1)**.
///
/// # Complexity
///
/// - **Time Complexity:** O(n), since we iterate from `2` to `n`.
/// - **Space Complexity:** O(1), as we only use two integer variables.
///
/// # Examples
///
/// ```
/// use leetcode_rust::problems::arrays_hashing::climb_stairs;
///
/// assert_eq!(climb_stairs(2), 2); // Ways: [1+1], [2]
/// assert_eq!(climb_stairs(3), 3); // Ways: [1+1+1], [1+2], [2+1]
/// assert_eq!(climb_stairs(4), 5); // Ways: [1+1+1+1], [1+1+2], [1+2+1], [2+1+1], [2+2]
/// ```
///
/// ```
/// use leetcode_rust::problems::arrays_hashing::climb_stairs;
///
/// assert_eq!(climb_stairs(1), 1); // Only 1 way: [1]
/// assert_eq!(climb_stairs(5), 8); // Fibonacci property: f(5) = f(4) + f(3)
/// ```
pub fn climb_stairs(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }

    let (mut prev, mut curr) = (1, 1);
    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }

    curr
}

/// LeetCode problem 53.
/// Finds the maximum sum of a contiguous subarray in `nums` using Kadane's Algorithm.
///
/// This function implements an efficient **O(n)** solution to the **maximum subarray problem**.
/// Given an array of integers, it finds the contiguous subarray with the largest sum
/// and returns that sum.
///
/// # Arguments
///
/// * `nums` - A `Vec<i32>` representing the input array of integers.
///
/// # Returns
///
/// Returns an `i32` representing the largest sum of any contiguous subarray.
///
/// # Approach
///
/// - Use a **greedy approach** (Kadane’s Algorithm).
/// - Maintain `running_sum`, the sum of the current subarray.
/// - If `running_sum` drops below `nums[i]`, reset it to `nums[i]`, effectively
///   starting a new subarray.
/// - Track the maximum sum encountered (`max_sum`).
///
/// # Complexity
///
/// - **Time Complexity:** O(n), since we iterate through `nums` once.
/// - **Space Complexity:** O(1), as we only use two integer variables.
///
/// # Examples
///
/// ```
/// use leetcode_rust::problems::arrays_hashing::max_sub_array;
///
/// let nums = vec![-2,1,-3,4,-1,2,1,-5,4];
/// assert_eq!(max_sub_array(nums), 6); // [4, -1, 2, 1] has the maximum sum of 6
/// ```
///
/// ```
/// use leetcode_rust::problems::arrays_hashing::max_sub_array;
///
/// let nums = vec![1];
/// assert_eq!(max_sub_array(nums), 1); // The only element is the max subarray
/// ```
///
/// ```
/// use leetcode_rust::problems::arrays_hashing::max_sub_array;
///
/// let nums = vec![-1, -2, -3, -4];
/// assert_eq!(max_sub_array(nums), -1); // Single element subarray [-1] is the max
/// ```
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut running_sum = nums[0];
    let mut max_sum = nums[0];

    for i in 1..nums.len() {
        running_sum = max(nums[i], running_sum + nums[i]);
        max_sum = max(running_sum, max_sum);
    }
    max_sum
}
