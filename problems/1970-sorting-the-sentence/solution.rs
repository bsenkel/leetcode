impl Solution {
    pub fn sort_sentence(s: String) -> String {
        let words: Vec<&str> = s.split_whitespace().collect();

        let mut result: Vec<String> = vec![String::new(); words.len()];

        for word in words {
            // -1 due to the 0-based index in the result array
            let pos = word.chars().last().unwrap().to_digit(10).unwrap() as usize - 1;

            // only the word
            result[pos] = word[..word.len() - 1].to_string();
        }
        
        result.join(" ")
    }
}
