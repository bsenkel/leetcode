// solution 1: use the integrated to_lowercase() method
// solution 2: manually convert with keeping the difference of 32 in mind (e.g. 'A' = 65, 'a' = 97)

impl Solution {
    pub fn to_lower_case(s: String) -> String {
        s.chars()
        .map(|c| {
            if c >= 'A' && c <= 'Z' {
                ((c as u8) + 32) as char
            } else {
                c
            }
        })
        .collect()
    }
}
