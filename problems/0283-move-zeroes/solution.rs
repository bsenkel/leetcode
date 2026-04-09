impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut last_non_zero_pos = 0;

        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums.swap(last_non_zero_pos, i);
                last_non_zero_pos += 1;
            }
        }
    }
}
