use std::collections::HashSet;

impl Solution {
    pub fn find_subarrays(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();
        for window in nums.windows(2) {
            let sum = window[0] + window[1];
            if set.contains(&sum) {
                return true;
            }
            set.insert(sum);
        }
        false
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(Solution::find_subarrays(vec![4, 2, 4]), true);
    }

    #[test]
    fn test02() {
        assert_eq!(Solution::find_subarrays(vec![1, 2, 3, 4, 5]), false);
    }

    #[test]
    fn test03() {
        assert_eq!(Solution::find_subarrays(vec![0, 0, 0]), true);
    }
}
