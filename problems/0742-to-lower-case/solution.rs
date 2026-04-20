// 'A' to 'Z': ASCII values 65–90
// 'a' to 'z': ASCII values 97–122
// diff = 32

impl Solution {
    pub fn to_lower_case(s: String) -> String {
        s.chars()
            .map(|c| match c {
                'A'..='Z' => (c as u8 + 32) as char,
                _ => c,
            })
            .collect()
    }
}
