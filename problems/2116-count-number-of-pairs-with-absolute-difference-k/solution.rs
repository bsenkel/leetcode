impl Solution {
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;

        for i in 0..nums.len() {
            for j in (i+1)..nums.len(){    
                let distance = (nums[i]-nums[j]).abs();
                if (distance == k) {
                    count += 1;
                }
            }
        }
        count
    }
}
