impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n <= 0 {
            return false;
        }

        let mut x = n;
        while x % 2 == 0 {
            x /= 2;
        }

        return x == 1
    }
}
