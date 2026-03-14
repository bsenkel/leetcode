impl Solution {
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        (0..nums.len())
            .flat_map(|i| (i + 1..nums.len()).map(move |j| (i, j)))
            .filter(|&(i, j)| nums[i].abs_diff(nums[j]) as i32 == k)
            .count() as i32
    }
}
