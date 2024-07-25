use std::cmp::max;
use std::cmp::min;

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
