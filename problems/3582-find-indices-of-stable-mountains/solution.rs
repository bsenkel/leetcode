impl Solution {
    pub fn stable_mountains(height: Vec<i32>, threshold: i32) -> Vec<i32> {
        let mut output = Vec::new();

        for i in 1..height.len() {
            if height[i - 1] > threshold {
                output.push(i as i32);
            }
        }

        output
    }
}
