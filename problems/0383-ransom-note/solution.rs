use std::collections::HashMap;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut magazine_count = HashMap::new();
        let mut ransom_count = HashMap::new();

        for c in magazine.chars() {
            *magazine_count.entry(c).or_insert(0) += 1;
        }

        for c in ransom_note.chars() {
            *ransom_count.entry(c).or_insert(0) += 1;
        }

        let mut can_construct = false;

        for (key, value) in ransom_count.iter() {
            match magazine_count.get(key) {
                Some(v) => {
                    if value <= v {
                        can_construct = true;
                    } else {
                        can_construct = false;
                        break;
                    }
                }
                None => {
                    can_construct = false;
                    break;
                },
            }
        }

        can_construct
    }
}
