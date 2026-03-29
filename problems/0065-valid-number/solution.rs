impl Solution {
    pub fn is_number(s: String) -> bool {
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();
        let mut i = 0;

        // skip optional sign
        if i < len && (chars[i] == '+' || chars[i] == '-') {
            i += 1;
        }

        // count digits before the dot
        let mut digits_before = 0;
        while i < len && chars[i].is_ascii_digit() {
            i += 1;
            digits_before += 1;
        }

        // optional decimal point + digits after it
        let mut digits_after = 0;
        if i < len && chars[i] == '.' {
            i += 1;
            while i < len && chars[i].is_ascii_digit() {
                i += 1;
                digits_after += 1;
            }
        }

        // check if there are at least some digits somewhere
        if digits_before == 0 && digits_after == 0 {
            return false;
        }

        // optional exponent
        if i < len && (chars[i] == 'e' || chars[i] == 'E') {
            i += 1;
            
            // optional sign in the exponent
            if i < len && (chars[i] == '+' || chars[i] == '-') {
                i += 1;
            }
            
            // the exponent requires at least one digit
            let mut exp_digits = 0;
            while i < len && chars[i].is_ascii_digit() {
                i += 1;
                exp_digits += 1;
            }

            if exp_digits == 0 {
                return false;
            }
        }

        // valid only if the entire string has been consumed
        i == len
    }
}
