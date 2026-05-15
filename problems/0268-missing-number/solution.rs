use std::collections::HashSet;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let set: HashSet<i32> = nums.into_iter().collect();
        
        (0..=n)
            .find(|x| !set.contains(x)).unwrap()
    }
}
