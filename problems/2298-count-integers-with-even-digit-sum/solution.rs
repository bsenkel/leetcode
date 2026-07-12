impl Solution {
    pub fn count_even(num: i32) -> i32 {
        let mut count = 0;

        for number in 1..=num {
            let mut sum = 0;

            for c in number.to_string().chars() {
                let digit = c.to_digit(10).unwrap();
                sum += digit as i32;
            }

            if sum % 2 == 0 {
                count += 1;
            }
        }

        count
    }
}
