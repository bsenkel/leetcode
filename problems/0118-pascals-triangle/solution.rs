impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut triangle: Vec<Vec<i32>> = Vec::new();

        for row in 0..num_rows {
            let mut new_row: Vec<i32> = Vec::new();

            for col in 0..=row {
                if col == 0 || col == row {
                    new_row.push(1);
                } else {
                    let top_left = triangle[(row - 1) as usize][(col - 1) as usize];
                    let top_right = triangle[(row - 1) as usize][col as usize];
                    new_row.push(top_left + top_right);
                }
            }

            triangle.push(new_row);
        }

        triangle
    }
}
