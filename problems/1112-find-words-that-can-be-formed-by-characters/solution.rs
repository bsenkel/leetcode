use std::collections::HashMap;

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut characters = HashMap::new();

        for c in chars.chars() {
            *characters.entry(c).or_insert(0) += 1;
        }

        let mut count = 0;

        for w in words{
            let mut good = true;
            let mut freq = HashMap::new();
            for c in w.chars(){
                *freq.entry(c).or_insert(0) += 1;
            }
            for (key, value) in freq.iter(){
                match characters.get(key){
                    Some(v) => {
                        if v < value {
                            good = false;
                            break;
                        }
                    }
                    None => {
                        good = false;
                        break;
                    }
                }
            }

            if good {
                count += w.chars().count();
            }
        }

        count as i32
    }
}
