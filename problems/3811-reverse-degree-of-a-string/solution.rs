use std::collections::HashMap;

impl Solution {
    pub fn reverse_degree(s: String) -> i32 {
        let lookup: HashMap<char, i32> = ('a'..='z')
            .zip((1..=26).rev())
            .collect();

        let mut sum = 0;
        for (index, c) in s.chars().enumerate() {
            sum += lookup.get(&c).unwrap() * (index as i32 + 1);
        }

        sum
    }
}
