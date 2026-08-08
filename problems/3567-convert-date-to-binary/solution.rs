impl Solution {
    pub fn convert_date_to_binary(date: String) -> String {
        let year: i32 = date[0..4].parse().unwrap();
        let month: i32 = date[5..7].parse().unwrap();
        let day: i32 = date[8..10].parse().unwrap();

        format!("{:b}-{:b}-{:b}", year, month, day)
    }
}
