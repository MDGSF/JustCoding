impl Solution {
    pub fn alternate_digit_sum(mut n: i32) -> i32 {
        let mut sum1 = 0;
        let mut sum2 = 0;
        let mut flag = 1;
        let mut count = 0;
        while n > 0 {
            let t = n % 10;
            n = n / 10;
            count += 1;

            sum1 += t * flag;
            sum2 += t * -flag;
            flag *= -1;
        }

        if count % 2 == 0 {
            sum2
        } else {
            sum1
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(Solution::alternate_digit_sum(521), 4);
        assert_eq!(Solution::alternate_digit_sum(111), 1);
        assert_eq!(Solution::alternate_digit_sum(886996), 0);
    }
}
