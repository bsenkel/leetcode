impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        if n == 1 {
            return true;
        }
        if n <= 0 {
            return false;
        }

        let mut number = n;

        for factor in [2, 3, 5]{
            while number % factor == 0{
                number /= factor;
            }
        }

        number == 1
    }
}
