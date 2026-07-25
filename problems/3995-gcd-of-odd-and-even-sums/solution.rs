impl Solution {
    pub fn gcd_of_odd_even_sums(n: i32) -> i32 {
        let sum_odd = n * n;
        let sum_even = n * (n + 1);
        Self::gcd(sum_odd, sum_even)
    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }
        a
    }
}
