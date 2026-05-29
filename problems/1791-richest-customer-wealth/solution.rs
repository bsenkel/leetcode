impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut max_wealth = 0;

        for customer in &accounts{
            let mut wealth = 0;

            for money in customer {
                wealth += money; 
            }

            if max_wealth < wealth {
                max_wealth = wealth;
            }
        }

        max_wealth
    }
}
