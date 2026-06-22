use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut counts: HashMap<char, usize> = HashMap::new();

        if s.chars().count() != t.chars().count() { return false; }

        for c in s.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }

        for c in t.chars() {
            if let Some(x) = counts.get_mut(&c){
                if *x != 0{
                    *x -= 1;
                }
                else {
                    return false;
                }
            }
            else{
                return false;
            }
        }

        true
    }
}
