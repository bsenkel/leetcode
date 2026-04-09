impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let k = k as usize;
        let mut chars: Vec<char> = s.chars().collect();
        let n = chars.len();

        for i in (0..n).step_by(2 * k) {
            let end;
            if i + k < n {
                end = i + k;
            } else {
                end = n;
            }
            
            chars[i..end].reverse();
        }

        chars.into_iter().collect()
    }
}
