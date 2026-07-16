use std::collections::HashSet;

impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let rows: [HashSet<char>; 3] = [
            "qwertyuiop".chars().collect(),
            "asdfghjkl".chars().collect(),
            "zxcvbnm".chars().collect(),
        ];

        words
            .into_iter()
            .filter(|word| {
                let lower: Vec<char> = word.chars().map(|c| c.to_ascii_lowercase()).collect();
                rows.iter().any(|row| lower.iter().all(|c| row.contains(c)))
            })
            .collect()
    }
}
