use std::collections::HashSet;

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let jewel_set: HashSet<char> = jewels.chars().collect();

        stones.chars()
            .filter(|stone| jewel_set.contains(stone))
            .count() as i32
    }
}
