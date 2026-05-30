impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut index = 0;

        for (pos, num) in nums.iter().enumerate(){
            if *num == target{
                index = pos;
                break;
            }
            else if *num > target{
                index = pos;
                break;
            }
            else if pos == nums.len() - 1 {
                index = pos + 1;
            }
        }

        index as i32
    }
}
