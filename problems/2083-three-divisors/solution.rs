impl Solution {
    pub fn is_three(n: i32) -> bool {
        let mut count = 1;

        for i in 1..= n/2 {
            if (n % i == 0) {
                count += 1;
            }

            if (count > 3) {
                return false; 
            }
        }

        return count == 3;
    }
}
