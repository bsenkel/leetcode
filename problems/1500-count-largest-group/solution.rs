use std::collections::HashMap;
impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut groups = HashMap::<i32, i32>::new();

        for i in 1..=n {
            let mut sum: i32 = 0;
            let mut x = i;
            while x > 0 {
                sum += x % 10;
                x /= 10;
            }
            
            *groups.entry(sum).or_insert(0) += 1;
        }

        let max = *groups.values().max().unwrap();
        let count = groups.values().filter(|&&v| v == max).count();

        count as i32
    }
}
