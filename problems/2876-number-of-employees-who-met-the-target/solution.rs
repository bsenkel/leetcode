impl Solution {
    pub fn number_of_employees_who_met_target(hours: Vec<i32>, target: i32) -> i32 {
        let mut targets = 0;

        for i in hours{
            if i >= target {
                targets += 1;
            }
        }

        targets
    }
}
