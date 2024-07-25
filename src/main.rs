mod problems;

use problems::longest_substring::{self};

fn main() {
    let string = String::from("abcdabca");
    let result = longest_substring::length_of_longest_substring(string);
    println!("Result for Problem: {:?}", result);
}
