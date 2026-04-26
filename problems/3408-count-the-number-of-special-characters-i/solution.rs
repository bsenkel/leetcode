use std::collections::HashSet;

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut lower = HashSet::new();
        let mut upper = HashSet::new();

        for c in word.chars() {
            if c.is_ascii_lowercase() {
                lower.insert(c);
            } else if c.is_ascii_uppercase() {
                upper.insert(c.to_ascii_lowercase());
            }
        }

        lower.iter().filter(|c| upper.contains(c)).count() as i32
    }
}
