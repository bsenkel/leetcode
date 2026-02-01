impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        let mut num = n as u32;
        let mut count = 0;

        while num != 0 {
            count += (num & 1) as i32;
            num >>= 1;
        }
        count
    }
}
