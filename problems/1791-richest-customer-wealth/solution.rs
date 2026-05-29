impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts
            .iter()
            .map(|customer| customer.iter().sum())
            .max()
            .unwrap_or(0)
    }
}
