impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let mut answer: Vec<i32> = Vec::new();

        for (i, number) in nums.iter().enumerate() {
            let sum_right: i32 = (nums[i + 1..]).iter().sum();
            let sum_left: i32 = (nums[..i]).iter().sum();

            answer.push((sum_left - sum_right).abs());
        }

        answer
    }
}
