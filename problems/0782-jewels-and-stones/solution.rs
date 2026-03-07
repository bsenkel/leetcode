// solution 1: with String.contains() 
//  - Total: O(n x m)
//  - Lookup of single char: O(m)
// solution 2: with HashSet 
//  - Total: O(n + m)
//  - Lookup of single char: O(1)

// n = number of stones
// m = number of jewels

use std::collections::HashSet;

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let jewel_set: HashSet<char> = jewels.chars().collect();

        stones.chars()
            .filter(|stone| jewel_set.contains(stone))
            .count() as i32
    }
}
