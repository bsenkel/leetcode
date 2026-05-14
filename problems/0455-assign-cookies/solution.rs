impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut greed = g;
        let mut size = s;

        greed.sort();
        size.sort();

        let (mut child, mut cookie) = (0, 0);

        while child < greed.len() && cookie < size.len() {
            if size[cookie] >= greed[child] {
                child += 1; // satisfied child
            }
            cookie += 1; // try next cookie
        }

        child as i32
    }
}
