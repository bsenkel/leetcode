impl Solution {
    pub fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
        const THRESH1: i32 = 10_000;
        const THRESH2: i64 = 1_000_000_000;

        let mut is_bulky = false;
        let mut is_heavy = false;

        let volume = i64::from(length) * i64::from(width) * i64::from(height);

        if length >= THRESH1
            || width >= THRESH1
            || height >= THRESH1
            || volume >= THRESH2
        {
            is_bulky = true;
        }

        if mass >= 100 {
            is_heavy = true;
        }

        if is_bulky && is_heavy {
            return "Both".to_string();
        }

        if is_bulky && !is_heavy {
            return "Bulky".to_string();
        }

        if !is_bulky && is_heavy {
            return "Heavy".to_string();
        }

        "Neither".to_string()
    }
}
