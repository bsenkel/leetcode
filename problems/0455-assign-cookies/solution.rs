impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut g = g;
        let mut s = s;

        g.sort(); // greed factor of children
        s.sort(); // cookie size

        let (mut child, mut cookie) = (0, 0);

        while child < g.len() && cookie < s.len() {
            if s[cookie] >= g[child] {
                child += 1; // satisfied child
            }
            cookie += 1; // try next cookie
        }

        child as i32 // number of satisfied children
    }
}
