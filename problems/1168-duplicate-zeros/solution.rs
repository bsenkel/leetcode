impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let n = arr.len();

        let zeros = arr.iter().filter(|&&x| x == 0).count();

        let mut i = n as isize - 1;
        let mut j = (n + zeros) as isize - 1;

        while i >= 0 {
            if j < n as isize {
                arr[j as usize] = arr[i as usize];
            }

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
