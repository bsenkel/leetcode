impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        (left..=right)
            .filter(|&num| {Self::check_self_dividing(num)})
            .collect()
    }

    // E.g. num = 128
    fn check_self_dividing(num: i32) -> bool {
        let mut n = num; // n = 128

        while n > 0 {
            let digit = n % 10; // 128 % 10 = 8
            
            if digit == 0 { // a self-dividing number is not allowed to contain the digit zero
                return false;
            }
            if num % digit != 0 { // 128 % 8 = 0
                return false;
            }

            n /= 10; // n = 128 / 10 = 12
        }
        
        true
    }
}
