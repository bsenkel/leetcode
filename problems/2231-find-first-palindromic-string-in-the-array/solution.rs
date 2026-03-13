impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        
        let mut output: String = "".to_string();
        for i in 0..words.len(){
            if words[i].chars().eq(words[i].chars().rev()){
                output = words[i].clone();
                break;
            }
        }

        output
    }
}
