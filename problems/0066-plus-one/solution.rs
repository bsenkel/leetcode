impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        for i in (0..digits.len()).rev() {
            if digits[i] < 9 {
                digits[i] += 1;
                return digits;
            }
            // if 9 -> 0, and transfer the increment to next digit
            digits[i] = 0;
        }
        // all digits are e.g. [9, 9, 9], therefore a new digit on the left end is needed
        let mut result = vec![1];
        result.extend(digits);
        result
    }
}
