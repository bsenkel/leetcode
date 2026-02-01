// solution 1: use the integrated count_ones() method
// solution 2: manually check every bit and move bits to the right

impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        let mut number = n as u32;
        let mut count = 0;

        while number != 0 {
            count += (number & 1) as i32; // check if the last bit is set to 1 via the AND-operator
            number >>= 1; // shift all bits to the right, therefore check the next bit
        }
        count
    }
}
