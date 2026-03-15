impl Solution {
    pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
        let person1 = (z-x).abs();
        let person2 = (z-y).abs();

        if (person1 < person2) {return 1;}
        if (person1 > person2) {return 2;}

        return 0
    }
}
