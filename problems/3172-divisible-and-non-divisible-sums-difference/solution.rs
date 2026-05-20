impl Solution {
    pub fn difference_of_sums(n: i32, m: i32) -> i32 {
        (1..=n)
            .map(|i| if i % m == 0 { -i } else { i })
            .sum()
    }
}
