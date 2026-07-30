impl Solution {
    pub fn reverse_prefix(s: String, k: i32) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        chars[..k as usize].reverse();
        chars.into_iter().collect()
    }
}
