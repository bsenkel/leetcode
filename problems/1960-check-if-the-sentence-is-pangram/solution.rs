use std::collections::HashSet;

impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let available_letters: HashSet<char> = ('a'..='z').collect();
        let content: HashSet<char> = sentence
            .chars()
            .collect();

        available_letters.is_subset(&content)
    }
}
