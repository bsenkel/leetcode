use std::collections::HashMap;

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut counts: HashMap<i32, i32> = HashMap::new();

        for num in arr {
            *counts.entry(num).or_insert(0) += 1;
        }

        let mut result = -1;

        for (num, count) in counts {
            if num == count && num > result {
                result = num;
            }
        }

        result
    }
}
