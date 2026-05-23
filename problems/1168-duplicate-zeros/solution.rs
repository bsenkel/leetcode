impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let n = arr.len();
        let mut i = 0;
        
        while i < n {
            if arr[i] == 0 {
                let mut j = n - 1;

                while j > i + 1 {
                    arr[j] = arr[j - 1];
                    j -= 1;
                }

                if i + 1 < n {
                    arr[i + 1] = 0;
                }

                i += 1;
            }

            i += 1;
        }
    }
}
