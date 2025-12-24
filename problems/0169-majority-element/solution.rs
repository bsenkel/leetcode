use std::collections::HashMap;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let threshold = nums.len() / 2;

        nums.iter()
            .fold(HashMap::new(), |mut map, &num| {
                *map.entry(num).or_insert(0) += 1;
                map
            })
            .into_iter()
            .find(|(_, count)| *count > threshold)
            .map(|(num, _)| num)
            .unwrap_or(0)
        }
}
