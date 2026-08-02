impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        details
            .iter()
            .filter(|p| {
                let age: i32 = p.chars().skip(11).take(2).collect::<String>().parse().unwrap();
                age > 60
            })
            .count() as i32
    }
}
