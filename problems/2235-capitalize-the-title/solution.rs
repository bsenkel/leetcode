impl Solution {
    pub fn capitalize_title(title: String) -> String {
        title
            .split(' ')
            .map(|word| {
                if word.len() <= 2 {
                    word.to_lowercase()
                } else {
                    let mut chars = word.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(first) => {
                            format!("{}{}", first.to_uppercase(), chars.as_str().to_lowercase())
                        }
                    }
                }
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}
