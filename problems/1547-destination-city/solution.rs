impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let start_points: std::collections::HashSet<&str> = paths.iter().map(|p| p[0].as_str()).collect();

        paths
            .iter()
            .find(|p| !start_points.contains(p[1].as_str()))
            .unwrap()[1]
            .clone()
    }
}
