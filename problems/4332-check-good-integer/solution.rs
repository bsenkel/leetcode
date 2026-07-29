impl Solution {
    pub fn check_good_integer(n: i32) -> bool {
        let mut digit_sum = 0;
        let mut square_sum = 0;

        let mut number = n;

        while number != 0 {
            let n = number % 10;
            digit_sum += n;
            square_sum += n * n;
            number /= 10;
        }

        square_sum - digit_sum >= 50   
    }
}
