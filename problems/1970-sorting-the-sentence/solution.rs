impl Solution {
    pub fn sort_sentence(s: String) -> String {
        let words: Vec<&str> = s.split_whitespace().collect();

        let n = words.len();
        let mut result: Vec<String> = vec![String::new(); n];

        for word in words {
            let pos = word.chars().last().unwrap().to_digit(10).unwrap() as usize - 1;

            let word_only = &word[..word.len() - 1];
            result[pos] = word_only.to_string();
        }
        
        result.join(" ")
    }
}
