// solution 1: use the integrated reverse() method
// solution 2: manually swap each element in-place, starting with the first and last element etc.

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut left = 0;
        let mut right = s.len() - 1;
        while left < right {
            s.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
}
