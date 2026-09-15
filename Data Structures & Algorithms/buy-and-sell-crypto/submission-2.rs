impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut l, mut r, mut global) = (0, 1, 0);
        
        while r < prices.len() {
            if prices[l] < prices[r] {
                global = std::cmp::max(global, prices[r] - prices[l]);
            } else {
                l = r;
            }
            r += 1;
        }

        global
    }
}
