impl Solution {
    pub fn minimum_operations(mut nums: Vec<i32>) -> i32 {
        let mut count = 0;
        while let Some(&min_num) = nums.iter().filter(|&&num| num > 0).min() {
            count += 1;
            for num in nums.iter_mut() {
                *num -= min_num;
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
        assert_eq!(Solution::minimum_operations(vec![1, 5, 0, 3, 5]), 3);
    }

    #[test]
    fn test02() {
        assert_eq!(Solution::minimum_operations(vec![0]), 0);
    }
}
