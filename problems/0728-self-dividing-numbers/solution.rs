impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        (left..=right)
            .filter(|&num| {
                let s = num.to_string();
                !s.contains('0') && 
                s.chars().all(|c| num % (c.to_digit(10).unwrap() as i32) == 0)
            })
            .collect()
    }
}
