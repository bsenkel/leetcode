impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let mut monday = 0;
        let mut total = 0;

        for i in 1..=n{
            let day_in_week = (i - 1) % 7 + 1;
            total += day_in_week + monday;

            if i % 7 == 0{
                monday += 1;
            } 

        }

        total
    }
}
