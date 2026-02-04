impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split(' ').map(|word| {
            let mut chars: Vec<char> = word.chars().collect();
            Self::reverse_string(&mut chars);
            chars.into_iter().collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
    }

    fn reverse_string(s: &mut Vec<char>) {
        if s.is_empty() {
            return;
        }
        let mut left = 0;
        let mut right = s.len() - 1;
        while left < right {
            s.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
}
