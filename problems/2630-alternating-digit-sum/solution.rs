impl Solution {
    pub fn alternate_digit_sum(n: i32) -> i32 {
        let number = n.to_string();
        let mut sum = 0;
        let mut flag = true;

        for c in number.chars() {
            if flag {
                sum += c.to_digit(10).unwrap() as i32;
                flag = false;
                continue;
            }
            if !flag {
                sum -= c.to_digit(10).unwrap() as i32;
                flag = true;
            }
        }

        sum
    }
}
