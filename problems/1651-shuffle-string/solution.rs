impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let mut result: Vec<char> = vec![' '; s.len()];
        let chars: Vec<char> = s.chars().collect();

        for (i, &j) in indices.iter().enumerate() {
            // result[4] = chars[0]
            // result[5] = chars[1]
            // result[6] = chars[2]
            // ...
            result[j as usize] = chars[i];
        }

        result.into_iter().collect()
    }
}
