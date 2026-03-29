impl Solution {
    pub fn is_number(s: String) -> bool {
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();
        let mut i = 0;

        if i < len && (chars[i] == '+' || chars[i] == '-') {
            i += 1;
        }

        let mut digits_before = 0;
        while i < len && chars[i].is_ascii_digit() {
            i += 1;
            digits_before += 1;
        }

        let mut digits_after = 0;
        if i < len && chars[i] == '.' {
            i += 1;
            while i < len && chars[i].is_ascii_digit() {
                i += 1;
                digits_after += 1;
            }
        }

        if digits_before == 0 && digits_after == 0 {
            return false;
        }

        if i < len && (chars[i] == 'e' || chars[i] == 'E') {
            i += 1;
            
            if i < len && (chars[i] == '+' || chars[i] == '-') {
                i += 1;
            }
            
            let mut exp_digits = 0;
            while i < len && chars[i].is_ascii_digit() {
                i += 1;
                exp_digits += 1;
            }

            if exp_digits == 0 {
                return false;
            }
        }

        i == len
    }
}
