impl Solution {
    pub fn alternating_sum(nums: Vec<i32>) -> i32 {
        let mut sum = 0;

        for (i, num) in nums.into_iter().enumerate(){
            if i % 2 == 0 {
                sum = sum + num;
            }
            else{
                sum = sum - num;
            }

        }

        sum

    }
}
