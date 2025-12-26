// Booyer Moore Voting Algorithm
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut candidate = 0;
        let mut count = 0;
        // find possible candidate
        for &num in &nums {
            if count == 0 {
                candidate = num;
            }
            count += if num == candidate { 1 } else { -1 };
        }
        // verification
        let mut verification_count = 0;
        for &num in &nums {
            if num == candidate {
                verification_count += 1;
            }
        }
        // check if candidate has majority
        if verification_count > nums.len() / 2 {
            candidate
        } else {
            panic!("No majority element found!")
        }
    }
}
