impl Solution {
    pub fn sum_of_multiples(n: i32) -> i32 {
        (1..=n)
            .filter(|i| i % 3 == 0 || i % 5 == 0 || i % 7 == 0)
            .sum()
    }
}
