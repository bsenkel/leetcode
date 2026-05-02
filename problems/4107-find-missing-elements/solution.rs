use std::collections::HashSet;
impl Solution {
    pub fn find_missing_elements(nums: Vec<i32>) -> Vec<i32> {
        let min = *nums.iter().min().unwrap();
        let max = *nums.iter().max().unwrap();
        let elements: HashSet<i32> = nums.into_iter().collect();

        (min..=max)
            .filter(|n| !elements.contains(n))
            .collect()
        }
}
