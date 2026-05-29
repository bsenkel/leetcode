use std::collections::HashSet;

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let allowed: HashSet<char> = allowed.chars().collect();

        words
            .iter()
            .filter(|word| word.chars().all(|c| allowed.contains(&c)))
            .count() as i32
    }
}
