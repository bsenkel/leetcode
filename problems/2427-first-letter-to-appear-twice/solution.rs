use std::collections::HashMap;

impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut counts: HashMap<char, usize> = HashMap::new();
        let mut letter: char = ' ';

        for c in s.chars() {
            *counts.entry(c).or_insert(0) += 1;

            if let Some(key) = counts.iter().find(|&(_, &v)| v == 2).map(|(k, _)| k) {
                letter = *key;
                break;
            }
        }

        letter
    }
}
