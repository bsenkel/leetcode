impl Solution {
    pub fn recover_order(order: Vec<i32>, friends: Vec<i32>) -> Vec<i32> {
        let mut finish = Vec::with_capacity(friends.len());

        for participant in order{
            if friends.contains(&participant){
                finish.push(participant);
            }

            if finish.len() == friends.len() {
                break;
            }
        }

        finish
    }
}
