use std::collections::{HashSet, HashMap};

impl Solution {
    pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        let set1: HashSet<i32> = nums1.into_iter().collect();
        let set2: HashSet<i32> = nums2.into_iter().collect();
        let set3: HashSet<i32> = nums3.into_iter().collect();

        let mut counts: HashMap<i32, usize> = HashMap::new();

        for set in [&set1, &set2, &set3] {
            for &number in set {
                *counts.entry(number).or_insert(0) += 1;
            }
        }

        let result: Vec<i32> = counts
            .into_iter()
            .filter(|(_, count)| *count >= 2)
            .map(|(number, _)| number)
            .collect();

        result
    }
}
