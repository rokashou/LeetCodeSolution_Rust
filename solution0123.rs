/* 123. Best Time to Buy and Sell Stock III */
/* Runtime: 2ms, Memory: 3.26MB */

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // Edge case: no price data means not profit possible
        if prices.is_empty() {
            return 0;
        }

        // State machine variables:
        // buy1: max profit after 1st buy (negative value, i.e., cash spent)
        // sell1: max profit after 1st sell
        // buy2: max profit after 2nd buy
        // sell2: max profit after 2nd sell (final answer)
        let mut buy1 = i32::MIN;
        let mut sell1 = 0;
        let mut buy2 = i32::MIN;
        let mut sell2 = 0;

        for &p in prices.iter() {
            // Update the state machine variables in order
            buy1 = buy1.max(-p);          // best (min) price to buy the 1st time
            sell1 = sell1.max(buy1 + p);  // best profit after 1st sell
            buy2 = buy2.max(sell1 - p);   // best profit after 2nd buy (reinvest 1st profit)
            sell2 = sell2.max(buy2 + p);  // best profit after 2nd sell
        }

        sell2
    }
}
