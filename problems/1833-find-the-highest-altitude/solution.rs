impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut max_altitude = 0;
        let mut current = 0;

        for g in gain{
            current += g;
            max_altitude = max_altitude.max(current);
        }

        max_altitude
    }
}
