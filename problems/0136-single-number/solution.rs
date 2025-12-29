use std::collections::HashSet;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut seen = HashSet::new();
        for num in nums {
            if !seen.remove(&num) {
                seen.insert(num);
            }
        }
        *seen.iter().next().unwrap()
    }
}
