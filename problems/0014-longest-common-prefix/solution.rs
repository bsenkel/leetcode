impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let Some(mut prefix) = strs.first().cloned() else {
            return String::new();
        };

        for s in strs.iter().skip(1) {
            while !s.starts_with(&prefix) {
                prefix.pop();

                if prefix.is_empty() {
                    return prefix;
                }
            }
        }

        prefix
    }
}
