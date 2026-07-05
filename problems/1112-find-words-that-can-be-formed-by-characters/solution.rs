use std::collections::HashMap;

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut characters = HashMap::new();

        for c in chars.chars() {
            *characters.entry(c).or_insert(0) += 1;
        }

        let mut total = 0;

        for word in words{
            let mut good = true;
            let mut freq = HashMap::new();
            
            for c in word.chars(){
                let count = freq.entry(c).or_insert(0);
                *count += 1;

                if *count > *characters.get(&c).unwrap_or(&0){
                    good = false;
                    break;
                }
            }

            if good {
                total += word.chars().count();
            }
        }

        total as i32
    }
}
