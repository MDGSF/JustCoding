impl Solution {
    pub fn common_factors(a: i32, b: i32) -> i32 {
        let mut count = 0;
        for i in 1..=(a.max(b)) {
            if a % i == 0 && b % i == 0 {
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
        assert_eq!(Solution::common_factors(12, 6), 4);
        assert_eq!(Solution::common_factors(25, 30), 2);
        assert_eq!(Solution::common_factors(885, 885), 8);
    }
}
