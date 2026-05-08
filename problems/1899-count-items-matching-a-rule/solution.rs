impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let idx = match rule_key.as_str() {
            "type"  => 0,
            "color" => 1,
            "name"  => 2,
            _       => return 0,
        };

        let mut count = 0;
        for item in &items {
            if let Some(val) = item.get(idx) {
                if val == &rule_value {
                    count += 1;
                }
            }
        }
        count as i32
    }
}
