use std::collections::HashMap;

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut counts: HashMap<char, usize> = HashMap::new();

        for c in s.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }

        let mut random: char = ' ';

        for c in t.chars() {
            match counts.get_mut(&c) {
                Some(count) if *count > 0 => *count -= 1,
                _ => random = c,
            }
        }

        random
    }
}
