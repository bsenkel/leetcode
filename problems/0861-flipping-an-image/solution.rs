impl Solution {
    pub fn flip_and_invert_image(mut image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        image
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .rev()
                    .map(|pixel| if pixel == 0 { 1 } else { 0 })
                    .collect()
            })
            .collect()
    }
}
