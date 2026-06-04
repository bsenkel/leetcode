use std::collections::HashMap;

impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let mut freq = HashMap::new();

        for n in &nums {
            *freq.entry(*n).or_insert(0) += 1;
        }

        let mut result = nums.clone();

        result.sort_by_key(|n| {
            let frequency = freq[n];
            let reverse_value = -n;

            (frequency, reverse_value)
        });

        result
    }
}
