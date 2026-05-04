impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut altitude = 0;
        let mut tmp = 0;

        for g in gain{
            tmp += g;
            if tmp > altitude{
                altitude = tmp;
            }
        }

        altitude
    }
}
