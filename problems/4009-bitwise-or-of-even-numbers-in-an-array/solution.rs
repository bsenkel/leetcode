impl Solution {
    pub fn even_number_bitwise_o_rs(nums: Vec<i32>) -> i32 {
        let result = nums.iter().filter(|&&x| x % 2 == 0).fold(0, |acc, x| acc | x);
        result
    }
}
