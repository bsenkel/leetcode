use std::collections::HashMap;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return Vec::new();
        }

        let mut result: Vec<String> = vec![String::new()];

        for digit in digits.chars() {
            let letters = Self::digit_to_letters(digit);
            let mut next: Vec<String> = Vec::new();

            for combo in &result {
                for letter in letters.chars() {
                    let mut new_combo = combo.clone();
                    new_combo.push(letter);
                    next.push(new_combo);
                }
            }

            result = next;
        }

        result
    }

    fn digit_to_letters(digit: char) -> &'static str {
        match digit {
            '2' => "abc",
            '3' => "def",
            '4' => "ghi",
            '5' => "jkl",
            '6' => "mno",
            '7' => "pqrs",
            '8' => "tuv",
            '9' => "wxyz",
            _ => "",
        }
    }
}
