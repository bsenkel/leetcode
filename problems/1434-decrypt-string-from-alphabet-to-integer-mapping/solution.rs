impl Solution {
    pub fn freq_alphabets(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let mut output = String::new();
        let mut i = 0;

        while i < chars.len() {
            let num = if i + 2 < chars.len() && chars[i + 2] == '#' {
                let tens = chars[i].to_digit(10).unwrap();
                let ones = chars[i + 1].to_digit(10).unwrap();
                i += 3;
                tens * 10 + ones
            } else {
                let digit = chars[i].to_digit(10).unwrap();
                i += 1;
                digit
            };

            output.push((b'a' + num as u8 - 1) as char);
        }

        output
    }
}
