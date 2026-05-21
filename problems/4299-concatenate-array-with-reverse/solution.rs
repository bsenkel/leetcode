impl Solution {
    pub fn concat_with_reverse(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = nums.clone();
        ans.extend(nums.into_iter().rev());
        ans
    }
}
