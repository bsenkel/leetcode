impl Solution {
    pub fn is_balanced(num: String) -> bool {
        let mut sum_even = 0;
        let mut sum_odd = 0;

        for (i, c) in num.chars().enumerate() {
            let digit = c.to_digit(10).unwrap();
            if i % 2 == 0 {
                sum_even += digit;
            } else {
                sum_odd += digit;
            }
        }
        sum_even == sum_odd
    }
}
