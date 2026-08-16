use std::collections::HashSet;

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let seen: HashSet<i32> = nums.iter().cloned().collect();
        (1..=nums.len() as i32)
            .filter(|n| !seen.contains(n))
            .collect()
    }
}
