impl Solution {
    pub fn stable_mountains(height: Vec<i32>, threshold: i32) -> Vec<i32> {
        let mut output = Vec::new();

        for (i,n) in height.iter().enumerate() {
            if i == 0 { 
                continue; 
            }

            if height[i - 1] > threshold {
                output.push(i as i32);
            }
        }

        output
    }
}
