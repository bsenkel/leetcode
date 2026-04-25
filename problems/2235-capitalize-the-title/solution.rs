impl Solution {
    pub fn capitalize_title(title: String) -> String {
        title
            .split(' ')
            .map(|word| {
                if word.len() <= 2 {
                    word.to_lowercase()
                } else {
                    Self::change_first_letter_to_uppercase_rest_lowercase(&word)
                }
            })
            .collect::<Vec<String>>()
            .join(" ")
    }

    fn change_first_letter_to_uppercase_rest_lowercase(s: &str) -> String{
        let mut chars = s.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => {
                format!("{}{}", first.to_uppercase(), chars.as_str().to_lowercase())
            }
        }
    }
}
