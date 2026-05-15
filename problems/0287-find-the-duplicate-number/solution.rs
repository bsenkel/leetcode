use std::collections::HashSet;

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut seen = HashSet::with_capacity(nums.len());

        *nums
            .iter()
            .find(|&&i| !seen.insert(i)).unwrap()
    }
}
