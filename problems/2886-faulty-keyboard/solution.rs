impl Solution {
    pub fn final_string(s: String) -> String {
        s.chars().fold(String::new(), |mut acc, c| {
            if c == 'i' {
                acc.chars().rev().collect()
            } else {
                acc.push(c);
                acc
            }
        })
    }
}
