impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let mut matches = 0;

        for item in &items {
            if let [t, c, n] = item.as_slice() {
                if rule_key == "type" && rule_value == *t{
                    matches += 1;
                }
                else if rule_key == "color" && rule_value == *c{
                    matches += 1;
                }
                else if rule_key == "name" && rule_value == *n{
                    matches += 1;
                }
            }
        }

        matches
    }
}
