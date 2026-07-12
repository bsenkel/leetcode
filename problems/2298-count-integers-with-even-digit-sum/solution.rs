impl Solution {
    pub fn count_even(num: i32) -> i32 {
        let mut count = 0;

        for i in 1..=num {
            let number = i.to_string();
            let mut sum = 0;
            for c in number.chars() {
                let d = c.to_digit(10).unwrap();
                sum += d as i32;
            }
            if sum % 2 == 0 {
                count += 1;
            }
        }

        count
    }
}
