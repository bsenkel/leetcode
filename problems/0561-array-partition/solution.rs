// In each pair (a,b) the larger value is wasted, only the minimum counts.
// The array needs to be sorted first, to minimize the waste across all pairs.
// Pair numbers that are close in value together.

impl Solution {
    pub fn array_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        nums.iter().step_by(2).sum()
    }
}
