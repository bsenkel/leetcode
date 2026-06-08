impl Solution {
    pub fn traffic_signal(timer: i32) -> String {
        match timer {
            x if x == 0 => "Green".to_string(),
            x if x == 30 => "Orange".to_string(),
            x if x > 30  && x <= 90 => "Red".to_string(),
            _ => "Invalid".to_string(),
        }
    }
}
