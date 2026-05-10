impl Solution {
    pub fn difference_of_sum(nums: Vec<i32>) -> i32 {      
        let element_sum: i32 = nums.iter().sum();
        
        let digit_sum: i32 = nums
            .iter()
            .flat_map(|n| n.to_string().into_bytes())
            .map(|b| (b - b'0') as i32)
            .sum();

        element_sum.abs_diff(digit_sum) as i32
    }
}
