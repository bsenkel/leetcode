impl Solution {
    pub fn capitalize_title(title: String) -> String {
        let mut result: Vec<String> = Vec::new();

        for word in title.split(' ') {
            if word.len() <= 2 {
                result.push(word.to_lowercase());
            }
            else
            {
                let mut s = String::new();
                for (i, c) in word.chars().enumerate() {
                    if i == 0 {
                        s.push(c.to_ascii_uppercase());
                    } else {
                        s.push(c.to_ascii_lowercase());
                    }
                }
                result.push(s);
            }
        }

        result.join(" ")
    }
}
