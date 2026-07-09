impl Solution {
    pub fn alternate_digit_sum(n: i32) -> i32 {
        let mut sum = 0;
        let mut sign = 1;

        for c in n.to_string().chars() {
            let digit = c.to_digit(10).unwrap() as i32;
            sum += digit * sign;
            sign *= -1;
        }

        sum
    }
}
