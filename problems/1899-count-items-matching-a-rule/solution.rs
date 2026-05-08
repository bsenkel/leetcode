impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let mut matches = 0;

        for item in &items {
            if let [typ, col, name] = item.as_slice() {
                if rule_key == "type" && rule_value == *typ{
                    matches += 1;
                }
                else if rule_key == "color" && rule_value == *col{
                    matches += 1;
                }
                else if rule_key == "name" && rule_value == *name{
                    matches += 1;
                }
            }
        }

        matches
    }
}
