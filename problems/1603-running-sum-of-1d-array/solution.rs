impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        nums.into_iter()
            .fold((Vec::new(), 0), |(mut result, current_sum), x| {
                let new_sum = current_sum + x;
                result.push(new_sum);
                (result, new_sum)
            })
            .0
    }
}
