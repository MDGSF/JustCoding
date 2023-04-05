impl Solution {
    pub fn count_digits(num: i32) -> i32 {
        let mut n = num;
        let mut count = 0;
        while n > 0 {
            let t = n % 10;
            n = n / 10;
            if num % t == 0 {
                count += 1;
            }
        }
        count
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(Solution::count_digits(7), 1);
        assert_eq!(Solution::count_digits(121), 2);
        assert_eq!(Solution::count_digits(1248), 4);
    }
}
