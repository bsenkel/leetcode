impl Solution {
    pub fn recover_order(order: Vec<i32>, friends: Vec<i32>) -> Vec<i32> {
        let mut finish = Vec::new();
        let participants = order.len() - 1;

        for i in 0..=participants{
            if friends.contains(&order[i]){
                finish.push(order[i]);
            }
        }

        finish
    }
}
