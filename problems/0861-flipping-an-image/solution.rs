impl Solution {
    pub fn flip_and_invert_image(mut image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for row in &mut image {
            row.reverse();
        }

        for y in 0..image.len() {
            for x in 0..image[y].len() {
                let pixel = image[y][x];
                
                if pixel == 1 { 
                    image[y][x] = 0;
                }
                if pixel == 0 { 
                    image[y][x] = 1;
                }
            }
        }

        image   
    }
}
