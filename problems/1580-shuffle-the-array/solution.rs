impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let n = n as usize;

        nums[..n]
            .iter()
            .zip(nums[n..].iter())
            .flat_map(|(&x, &y)| [x, y])
            .collect()
        }
}
