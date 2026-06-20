impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let rows = img.len();
        let cols = img[0].len();

        let mut result = vec![vec![0; cols]; rows];

        for row in 0..rows {
            for col in 0..cols {
                let mut sum = 0;
                let mut count = 0;

                // keep the 3x3 filter within the image bounds
                let row_start = row.saturating_sub(1);
                let row_end = (row + 1).min(rows - 1);

                let col_start = col.saturating_sub(1);
                let col_end = (col + 1).min(cols - 1);

                // sum and count all pixels within the filter bounds
                for neighbor_row in row_start..=row_end {
                    for neighbor_col in col_start..=col_end {
                        sum += img[neighbor_row][neighbor_col];
                        count += 1;
                    }
                }

                result[row][col] = sum / count;
            }
        }
        
        result
    }
}
