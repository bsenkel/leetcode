impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }
        
        format!("{s}{s}").contains(&goal)
    }
}
