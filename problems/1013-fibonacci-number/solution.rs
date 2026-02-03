// solution 1: recursive (slow)
// solution 2: iterative (faster)

impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n <= 1 {
            return n;
        }

        let mut prev = 0;
        let mut current = 1;
        for _ in 2..=n {
            let next = prev + current;
            prev = current;
            current = next;
        }
        current
    }
}
