impl Solution {
    pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
        let person1 = x.abs_diff(z);
        let person2 = y.abs_diff(z);

        if person1 < person2 {
            1
        } else if person1 > person2 {
            2
        } else {
            0
        } 
    }
}
