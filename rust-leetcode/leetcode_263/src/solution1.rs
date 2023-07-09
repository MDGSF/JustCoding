impl Solution {
    pub fn is_ugly(mut n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        while n % 2 == 0 {
            n /= 2;
        }
        while n % 3 == 0 {
            n /= 3;
        }
        while n % 5 == 0 {
            n /= 5;
        }
        n == 1
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(Solution::is_ugly(6), true);
        assert_eq!(Solution::is_ugly(1), true);
        assert_eq!(Solution::is_ugly(14), false);
    }
}
