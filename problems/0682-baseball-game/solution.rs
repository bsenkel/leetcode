impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut record: Vec<i32> = Vec::new();

        for op in operations {
            match op.as_str() {
                "C" => {
                    // delete last score
                    record.pop();
                }
                "D" => {
                    // double the previous score and add it
                    if let Some(&last) = record.last() {
                        record.push(last * 2);
                    }
                }
                "+" => {
                    // sum previous two scores and add it
                    let len = record.len();
                    if len >= 2 {
                        let sum = record[len - 1] + record[len - 2];
                        record.push(sum);
                    }
                }
                _ => {
                    // add a new score of x
                    if let Ok(num) = op.parse::<i32>() {
                        record.push(num);
                    }
                }
            }
        }

        record.iter().sum()
    }
}
