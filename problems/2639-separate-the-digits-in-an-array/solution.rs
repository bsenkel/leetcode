impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        let mut digits: Vec<i32> = Vec::new();

        for num in nums {
            let number = num.to_string();
            for c in number.chars() {
                let d = c.to_digit(10).unwrap();
                digits.push(d as i32);
            }
        }

        digits
    }
}
