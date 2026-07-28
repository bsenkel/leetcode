impl Solution {
    pub fn remove_trailing_zeros(num: String) -> String {
        let mut remove = 0;
        
        for c in num.chars().rev() {
            if c != '0' {
                break;
            }
            remove += 1;
        }

        num.chars().take(num.chars().count() - remove).collect()
    }
}
