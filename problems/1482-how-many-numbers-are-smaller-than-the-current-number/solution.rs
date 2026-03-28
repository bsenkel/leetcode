impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut sorted = nums.clone();
        sorted.sort();

        let mut result = Vec::with_capacity(nums.len());

        for num in nums {
            let count = sorted.iter().position(|&x| x == num).unwrap();
            result.push(count as i32);
        }

        result
    }
}
