impl Solution {
    pub fn count_even(num: i32) -> i32 {
        (1..=num)
          .filter(|&n| Self::digit_sum(n) % 2 == 0)
          .count() as i32
    }

    fn digit_sum(mut n: i32) -> i32 {
        let mut sum = 0;

        while n > 0 {
            sum += n % 10;
            n /= 10;
        }

        sum
    }
}
