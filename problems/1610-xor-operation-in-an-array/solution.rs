impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        (0..n)
            .map(|i| start + 2 * i)
            .reduce(|acc, x| acc ^ x)
            .unwrap_or(0)
    }
}
