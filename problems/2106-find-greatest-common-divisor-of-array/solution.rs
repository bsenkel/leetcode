impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        let mut min = nums.iter().copied().min().unwrap();
        let mut max = nums.iter().copied().max().unwrap();

        while min != 0 {
            let rest = max % min;

            max = min;
            min = rest;
        }

        max
    }
}
