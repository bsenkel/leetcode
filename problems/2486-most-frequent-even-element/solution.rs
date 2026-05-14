use std::collections::HashMap;

impl Solution {
    pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
        let mut frequent: HashMap<i32, i32> = HashMap::new();
        let mut result = -1;
        let mut max_count = 0;

        for n in nums {
            if n % 2 == 0 {
                *frequent.entry(n).or_insert(0) += 1;
            }
        }

        for (&num, &count) in &frequent {
            if count > max_count || (count == max_count && num < result) {
                max_count = count;
                result = num;
            }
        }

        result
    }
}
