use std::collections::HashMap;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut count1 = HashMap::new();
        let mut count2 = HashMap::new();

        for c in magazine.chars() {
            *count1.entry(c).or_insert(0) += 1;
        }

        for c in ransom_note.chars() {
            *count2.entry(c).or_insert(0) += 1;
        }

        let mut can_do = false;

        for (key, value) in count2.iter() {
            match count1.get(key) {
                Some(v) => {
                    if value <= v {
                        can_do = true;
                    } else {
                        can_do = false;
                        break;
                    }
                }
                None => {
                    can_do = false;
                    break;
                },
            }
        }

        can_do
    }
}
