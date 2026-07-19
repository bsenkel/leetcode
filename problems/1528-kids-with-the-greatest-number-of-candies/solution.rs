impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let mut output: Vec<bool> = Vec::new();
        let max = candies.iter().max().unwrap();

        for i in candies.iter() {
            if i + extra_candies >= *max {
                output.push(true);
            } else {
                output.push(false);
            }
        }

        output
    }
}
