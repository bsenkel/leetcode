impl Solution {
    pub fn replace_digits(s: String) -> String {
        let bytes: Vec<u8> = s.bytes().collect();
        let mut result: Vec<u8> = Vec::new();

        for (index, byte) in bytes.iter().enumerate() {
            if index % 2 == 0 {
                result.push(*byte);
            } else {
                // odd indices
                let shift = byte - b'0';
                let prev = result[index - 1];
                result.push(prev + shift);
            }
        }

        String::from_utf8(result).unwrap()
    }
}
