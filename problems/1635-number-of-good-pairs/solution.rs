use std::collections::HashMap;

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut counts: HashMap<i32, i32> = HashMap::new();
        let mut pairs = 0;

        for num in nums {
            let count = counts.entry(num).or_insert(0);

            pairs += *count;
            *count += 1;
        }

        pairs
    }
}
