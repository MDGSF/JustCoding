impl Solution {
    pub fn the_maximum_achievable_x(num: i32, t: i32) -> i32 {
        num + t * 2
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(Solution::the_maximum_achievable_x(4, 1), 6);
        assert_eq!(Solution::the_maximum_achievable_x(3, 2), 7);
    }
}
