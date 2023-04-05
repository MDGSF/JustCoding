impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        let mut left = 0;
        let mut right = n * (1 + n) / 2;
        for i in 1..=n {
            left += i;
            right -= i - 1;
            if left == right {
                return i;
            }
        }
        -1
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(Solution::pivot_integer(8), 6);
        assert_eq!(Solution::pivot_integer(1), 1);
        assert_eq!(Solution::pivot_integer(4), -1);
    }
}
