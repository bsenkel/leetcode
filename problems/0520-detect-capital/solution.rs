impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        Self::is_all_uppercase(&word)
            || Self::is_all_lowercase(&word)
            || Self::is_first_letter_capital_rest_lowercase(&word)
    }

    fn is_all_uppercase(s: &str) -> bool {
        s.chars().all(|c| c.is_uppercase())
    }

    fn is_all_lowercase(s: &str) -> bool {
        s.chars().all(|c| c.is_lowercase())
    }

    fn is_first_letter_capital_rest_lowercase(s: &str) -> bool {
        let mut chars = s.chars();
        match chars.next() {
            Some(first) if first.is_uppercase() => {
                chars.all(|c| c.is_lowercase())
            }
            _ => false,
        }
    }
}
