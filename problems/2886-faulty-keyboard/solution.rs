impl Solution {
    pub fn final_string(s: String) -> String {
        let mut output = String::new();

        for c in s.chars() {
            if c == 'i' {
                output = output.chars().rev().collect();
            } else {
                output.push(c);
            }
        }

        output
    }
}
