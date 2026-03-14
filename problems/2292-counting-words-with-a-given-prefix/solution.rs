impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        words
            .iter()
            .map(|word| word.starts_with(&pref) as i32)
            .sum()
    }
}
