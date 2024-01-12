/**
 * You are given an integer array 'prices' where 'prices[i]' is the price of a given stock
 * on the ith day. On each day, you may decide to buy and/or sell the stock. You can only
 * hold at most one share of the stock at any time. However, you can buy it then immediately
 * sell it on the same day. Find and return the maximum profit you can achieve.
 */
fn main() {
    let prices1 = vec![7, 1, 5, 3, 6, 4]; // 7
    let prices2 = vec![1, 2, 3, 4, 5]; // 4
    let prices3 = vec![7, 6, 4, 3, 1]; // 0

    println!("{}", Solution::max_profit(prices1));
    println!("{}", Solution::max_profit(prices2));
    println!("{}", Solution::max_profit(prices3));
}

struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // starting at zero profit, go day by day
        // if the price is higher than the previous day, buy and sell
        // if the price is lower than the previous day, don't buy or sell

        let mut profit = 0;

        // iterate over vector starting at index 1
        for i in 1..prices.len() {
            // if current index price is greater than previous index price
            if prices[i] > prices[i - 1] {
                // we bought yesterday and sold today as needed
                // line goes up
                profit += prices[i] - prices[i - 1];
            }
        }
        profit
    }
}