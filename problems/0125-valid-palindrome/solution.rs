impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let filtered: Vec<char> = s
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();

        let len = filtered.len();

        filtered.iter()
            .take(len/2)
            .zip(filtered.iter().rev().take(len/2))
            .all(|(a, b)| a == b)
    }
}
