impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let mut sorted = nums.to_vec();
        sorted.sort();
        
        let n = nums.len();

        // max top3 numbers
        let c1 = sorted[n-1] * sorted[n-2] * sorted[n-3];
        
        // two lowest numbers and the largest number
        let c2 = sorted[0] * sorted[1] * sorted[n-1];
        
        c1.max(c2)
    }
}
