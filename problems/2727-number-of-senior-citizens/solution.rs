impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        let mut sum = 0;

        for p in details {
            let age_string: String = p.chars().skip(11).take(2).collect();
            let age: i32 = age_string.parse().unwrap();
            if age > 60 {
                sum += 1;
            }
        }

        sum
    }
}
