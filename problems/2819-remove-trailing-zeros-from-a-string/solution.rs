impl Solution {
    pub fn remove_trailing_zeros(num: String) -> String {
        let mut iter = num.chars().rev();

        let mut count = 0;
        let mut flag = true;
        
        while flag{
            if iter.next() == Some('0') {
                count += 1;
            } else {
                flag = false;
            }
        }
        
        let result: String = num.chars().take(num.chars().count() - count).collect();

        result
    }
}
