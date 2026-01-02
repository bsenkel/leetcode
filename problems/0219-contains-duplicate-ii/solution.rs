use std::collections::HashSet;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let mut seen = HashSet::new();

        for (i, &num) in nums.iter().enumerate() {
            if i > k {
                seen.remove(&nums[i - k - 1]);
            }
            if !seen.insert(num) {
                return true;
            }
        }

        false
    }
}
