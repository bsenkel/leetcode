impl Solution {
    pub fn earliest_time(tasks: Vec<Vec<i32>>) -> i32 {
        let mut sums = Vec::new();

        for task in &tasks{
            let sum = task[0] + task[1];
            sums.push(sum);
        }
        
        *sums.iter().min().unwrap()
    }
}
