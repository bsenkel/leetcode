impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut unique_elements = Vec::<i32>::new();

        for num in &nums{
            let once = nums.iter().filter(|x| **x == *num).take(2).count() == 1;
            if once {
                unique_elements.push(*num);
            }
        }

        unique_elements.iter().sum()
    }
}
