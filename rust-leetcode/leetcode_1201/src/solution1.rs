impl Solution {
    pub fn nth_ugly_number(n: i32, a: i32, b: i32, c: i32) -> i32 {
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(Solution::nth_ugly_number(3, 2, 3, 5), 4);
    }

    #[test]
    fn test02() {
        assert_eq!(Solution::nth_ugly_number(4, 2, 3, 4), 6);
    }
}
