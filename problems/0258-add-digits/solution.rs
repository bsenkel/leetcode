impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        
        let mut number = num;
        
        while number >= 10 {
            let mut sum = 0;

            while number > 0 {
                sum += number % 10;
                number /= 10;
            }

            number = sum;
        }

        number
    }
}
