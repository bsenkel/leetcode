impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums.len());

        for i in 0..nums.len() {
            let mut count = 0;
            for x in nums[i + 1..].iter() {
                if *x < nums[i] {
                    count += 1;
                }
            }
            result.push(count);
        }

        result
    }
}
