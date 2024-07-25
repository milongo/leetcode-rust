use std::cmp::max;
use std::collections::HashSet;

pub fn length_of_longest_substring(s: String) -> i32 {
    
    let mut longest_substring:i32 = 0;
    let mut start = 0;
    let mut seen: HashSet<char> = HashSet::new();
    let s_chars: Vec<char> = s.chars().collect();

    for stop in 0..s.len() {

        while seen.contains(&s_chars[stop]) {
            seen.remove(&s_chars[start]);
            start += 1;
        }
        
        let window_length:i32 = ((stop - start) as i32) + 1;
        longest_substring = max(window_length, longest_substring);
        seen.insert(s_chars[stop]);
    }
    longest_substring
}