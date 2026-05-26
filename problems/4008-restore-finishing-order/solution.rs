impl Solution {
    pub fn recover_order(order: Vec<i32>, friends: Vec<i32>) -> Vec<i32> {
        let ids = friends.len();
        let mut finish_order = Vec::with_capacity(ids);

        for participant in order{
            if friends.contains(&participant){
                finish_order.push(participant);
            }
            if finish_order.len() == ids {
                break;
            }
        }

        finish_order
    }
}
