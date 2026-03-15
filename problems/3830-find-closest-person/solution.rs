impl Solution {
    pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
        let dist_x = x.abs_diff(z);
        let dist_y = y.abs_diff(z);

        if dist_x < dist_y {
            1
        } else if dist_x > dist_y {
            2
        } else {
            0
        } 
    }
}
