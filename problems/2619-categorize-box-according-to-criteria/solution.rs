impl Solution {
    pub fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
        const DIMENSION_THRESHOLD: i32 = 10_000;
        const VOLUME_THRESHOLD: i64 = 1_000_000_000;
        const MASS_THRESHOLD: i32 = 100;

        let mut is_bulky = false;
        let mut is_heavy = false;

        let volume = i64::from(length) * i64::from(width) * i64::from(height);

        if length >= DIMENSION_THRESHOLD
            || width >= DIMENSION_THRESHOLD
            || height >= DIMENSION_THRESHOLD
            || volume >= VOLUME_THRESHOLD
        {
            is_bulky = true;
        }

        if mass >= MASS_THRESHOLD {
            is_heavy = true;
        }

        match (is_bulky, is_heavy) {
            (true, true) => "Both".to_string(),
            (true, false) => "Bulky".to_string(),
            (false, true) => "Heavy".to_string(),
            (false, false) => "Neither".to_string(),
        }
    }
}
