use std::collections::HashMap;

impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut counts: HashMap<char, usize> = HashMap::new();
        let mut letter = ' ';

        for c in s.chars() {
            let count = counts.entry(c).or_insert(0);
            *count += 1;

            if *count == 2 {
                letter = c;
                break;
            }
        }

        letter
    }
}
