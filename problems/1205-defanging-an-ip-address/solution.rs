impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        let mut result = String::with_capacity(address.len() + 2 * address.matches('.').count());

        for ch in address.chars() {
            if ch == '.' {
                result.push_str("[.]");
            } else {
                result.push(ch);
            }
        }

        result
    }
}
