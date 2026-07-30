impl Solution {
    pub fn reverse_prefix(s: String, k: i32) -> String {
        let mut characters: Vec<char> = s.chars().collect();

        let k = k as usize;
        characters[..k].reverse();
        characters.iter().collect()
    }
}
