impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let n = arr.len();

        // count how many extra slots would be needed after duplicating zeros
        let zeros = arr.iter().filter(|&&x| x == 0).count();

        // i points to the original array
        // j to the virtual expanded array
        let mut i = n as isize - 1;
        let mut j = (n + zeros) as isize - 1;

        // work backwards, so values, which still need to be read do not get overwritten
        while i >= 0 {
            // write only if the virtual position is inside the real array
            if j < n as isize {
                arr[j as usize] = arr[i as usize];
            }

            // a zero occupies two positions in the virtual array
            if arr[i as usize] == 0 {
                j -= 1;

                if j < n as isize {
                    arr[j as usize] = 0;
                }
            }

            i -= 1;
            j -= 1;
        }
    }
}
