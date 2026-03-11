impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut number = 0;

        for i in 0..operations.len() {
            match operations[i].as_str(){
                "++X" => number += 1,
                "X++" => number += 1,
                "--X" => number -= 1,
                "X--" => number -= 1,
                _ => {},
            }
        }

        number
    }
}
