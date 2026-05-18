impl Solution {
    pub fn mirror_distance(n: i32) -> i32 {
        let n_reverse: i32 = n
            .to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse()
            .unwrap();
        
        (n - n_reverse).abs()
    }
}
