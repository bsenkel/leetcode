impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        for (index, &price) in prices.iter().enumerate() {
            let next_price = prices.get(index + 1);
            let next = match next_price {
                Some(value) => value,
                None => &0,
            };
            if price < *next {
                max_profit += *next - price;
            }
        }
        max_profit
    }
}
