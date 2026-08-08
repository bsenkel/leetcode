impl Solution {
    pub fn convert_date_to_binary(date: String) -> String {
        let year_string: String = date.chars().take(4).collect();
        let year: i32 = year_string.parse().unwrap();

        let month_string: String = date.chars().skip(5).take(2).collect();
        let month: i32 = month_string.parse().unwrap();

        let day_string: String = date.chars().skip(8).take(2).collect();
        let day: i32 = day_string.parse().unwrap();

        format!("{:b}-{:b}-{:b}", year, month, day)
    }
}
