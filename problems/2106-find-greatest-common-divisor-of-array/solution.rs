impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        let mut min = nums.iter().copied().min().unwrap();
        let mut max = nums.iter().copied().max().unwrap();

        let mut rest = max;

        while rest != 0 {
            rest = max % min;

            if rest == 0 {
                return min;
            }

            max = min;
            min = rest;
        }

        rest as i32
    }
}
