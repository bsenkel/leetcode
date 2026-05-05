impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut unique_elements = Vec::<i32>::new();

        for num in &nums{
            if Self::check(&nums, *num){
                unique_elements.push(*num);
            }
        }

        unique_elements.iter().sum()
    }

    fn check(vec: &Vec<i32>, value: i32) -> bool {
        vec.iter().filter(|&&x| x == value).take(2).count() == 1
    }
}
