impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        items.iter().filter(|item| {
            if let [typ, color, name] = item.as_slice() {
                match rule_key.as_str() {
                    "type" => *typ == rule_value,
                    "color" => *color == rule_value,
                    "name" => *name == rule_value,
                    _ => false,
                }
            } else {
                false
            }
        }).count().try_into().unwrap()
    }
}
