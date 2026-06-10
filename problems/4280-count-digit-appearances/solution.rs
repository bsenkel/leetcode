impl Solution {
    pub fn count_digit_occurrences(nums: Vec<i32>, digit: i32) -> i32 {
        let mut count = 0;

        for num in nums{
            let mut number = num;
            
            while number > 0 {
                let x = number % 10;
                
                if x == digit {
                    count += 1;
                }
                
                number /= 10;
            }
        }

        count
    }
}
