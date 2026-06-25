use std::collections::HashMap;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut counts = HashMap::new();

        for c in s.chars() {
            let mut count = counts.entry(c).or_insert(0);
            *count += 1;
        }

        let mut index = 0;
        let mut found = false;

        for c in s.chars() {
            if counts.get(&c) == Some(&1) {
                found = true;
                break; 
            }
            index += 1;
        }

        if !found {
            -1
        } else {
            index
        }
    }
}
