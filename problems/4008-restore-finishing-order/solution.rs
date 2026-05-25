impl Solution {
    pub fn recover_order(order: Vec<i32>, friends: Vec<i32>) -> Vec<i32> {
        let mut finish = Vec::new();

        for participant in order{
            if friends.contains(&participant){
                finish.push(participant);
            }
        }

        finish
    }
}
