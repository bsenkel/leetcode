use std::collections::HashMap;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut counts = HashMap::new();

        for c in s.chars() {
            let mut count = counts.entry(c).or_insert(0);
            *count += 1;
        }

        for (index, c) in s.chars().enumerate() {
            if counts.get(&c) == Some(&1) {
                return index as i32;
            }
        }

        -1
    }
}
