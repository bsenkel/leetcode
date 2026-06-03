impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut pairs: Vec<(i32, String)> = heights
            .into_iter()
            .zip(names.into_iter())
            .collect();
        
        pairs.sort();
        pairs.reverse();
        pairs.into_iter().map(|(_, name)| name).collect()
    }
}
