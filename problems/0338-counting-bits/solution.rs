impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut output: Vec<i32> = Vec::new();
        
        for i in 0..=n {
            let mut number = i;
            let mut count = 0;
            
            while number > 0 {
                if number & 1 == 1 {
                    count += 1;
                }
                number >>= 1;
            }

            output.push(count as i32);
        }

        output
    }
}
