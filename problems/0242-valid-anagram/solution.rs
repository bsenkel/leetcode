use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.chars().count() != t.chars().count() { 
            return false; 
        }
        
        let mut counts: HashMap<char, usize> = HashMap::new();

        for c in s.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }

        for c in t.chars() {
            match counts.get_mut(&c){
                Some(x) if *x != 0 => *x -= 1,
                _ => return false,
            }            
        }

        true
    }
}
