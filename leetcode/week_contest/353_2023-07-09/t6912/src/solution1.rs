impl Solution {
    pub fn max_non_decreasing_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(
            Solution::max_non_decreasing_length(vec![2, 3, 1], vec![1, 2, 1]),
            2
        );
    }

    #[test]
    fn test02() {
        assert_eq!(
            Solution::max_non_decreasing_length(vec![1, 3, 2, 1], vec![2, 2, 3, 4]),
            4
        );
    }
}
