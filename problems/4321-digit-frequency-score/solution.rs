use std::collections::HashMap;

impl Solution {
    pub fn digit_frequency_score(n: i32) -> i32 {
        let mut freq: HashMap<i32, usize> = HashMap::new();
        let mut x = n;

        while x > 0 {
            let mut d = x % 10;
            *freq.entry(d).or_insert(0) += 1;
            x /= 10;
        }

        freq.iter().map(|(a,b)| a * *b as i32).sum()
    }
}
