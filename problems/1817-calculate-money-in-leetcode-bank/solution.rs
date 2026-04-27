impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let mut week_offset = 0;
        let mut total = 0;

        for i in 1..=n{
            let day_in_week = (i - 1) % 7 + 1;
            total += day_in_week + week_offset;

            if i % 7 == 0{
                week_offset += 1;
            }
        }

        total
    }
}
