impl Solution {
    pub fn check_good_integer(n: i32) -> bool {
        let mut digitSum = 0;
        let mut squareSum = 0;

        let mut number = n;

        while number != 0 {
            let n = number % 10;
            digitSum += n;
            squareSum += n * n;
            number /= 10;
        }

        squareSum - digitSum >= 50   
    }
}
