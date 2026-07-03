impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut answer = Vec::new();

        for i in 1..=n {
            match (i % 3, i % 5) {
                (0, 0) => answer.push("FizzBuzz".to_string()),
                (0, _) => answer.push("Fizz".to_string()),
                (_, 0) => answer.push("Buzz".to_string()),
                _ => answer.push(i.to_string()),
            }
        }

        answer
    }
}
