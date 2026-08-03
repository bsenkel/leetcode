impl Solution {
    pub fn min_element(nums: Vec<i32>) -> i32 {
        let mut sums: Vec<i32> = Vec::new();

        for e in nums {
            let mut sum = 0;
            let mut n = e;
            while n != 0 {
                sum += n % 10;
                n /= 10;
            }
            sums.push(sum);
        }

        *sums.iter().min().unwrap()
    }
}
