use std::collections::HashSet;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut seen = HashSet::new();
        let mut num = n;

        while num != 1 && !seen.contains(&num) {
            seen.insert(num);
            num = Self::sum_of_squares_of_digits(num);
        }

        num == 1
    }

    fn sum_of_squares_of_digits(mut num: i32) -> i32 {
        let mut sum = 0;
        while num > 0 {
            let digit = num % 10;
            sum += digit * digit;
            num /= 10;
        }
        sum
    }
}
