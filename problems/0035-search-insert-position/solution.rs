impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        for (pos, num) in nums.iter().enumerate(){
            if *num >= target{
                return pos as i32;
            }
        }

        nums.len() as i32
    }
}
