fn main() {
    /*
        You are given an array of integers 'prices', where 'prices[i]' is the price 
        of a given stock on the 'ith' day. You want to maximize your profit by choosing
        a single day to buy one stock and choosing a different day in the future to sell
        that stock.

        Return the maximum profit you can achieve from this transaction. If you cannot
        achieve any profit, return 0.
     */

    let prices = vec![7,1,5,3,6,4];
    let result = Solution::max_profit(prices);
    println!("result: {}", result);

    let prices = vec![7,6,4,3,1];
    let result = Solution::max_profit(prices);
    println!("result: {}", result);
}


struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut lowest_price: i32 = std::i32::MAX;
        let mut best_profit: i32 = 0;

        for value in &prices {
            if *value < lowest_price {
                lowest_price = *value; // update lowest price for each iteration
            } else if (*value - lowest_price) > best_profit {
                best_profit = *value - lowest_price; // set best profit if it's greater than current best profit
            }
        }

        // this returns zero for a strictly decreasing vector,
        // because the best profit is never greater than zero
        best_profit
    }
}