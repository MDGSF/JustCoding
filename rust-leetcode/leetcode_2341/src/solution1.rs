use std::collections::HashMap;

impl Solution {
    pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
        let mut m = HashMap::new();
        for &num in nums.iter() {
            *m.entry(num).or_default() += 1;
        }

        let mut result = vec![0, 0];
        for (_num, count) in m.iter() {
            result[0] += count / 2;
            if count % 2 != 0 {
                result[1] += 1;
            }
        }
        result
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(
            Solution::number_of_pairs(vec![1, 3, 2, 1, 3, 2, 2]),
            vec![3, 1]
        );
    }

    #[test]
    fn test02() {
        assert_eq!(
            Solution::number_of_pairs(vec![1, 1]),
            vec![1, 0]
        );
    }

    #[test]
    fn test03() {
        assert_eq!(
            Solution::number_of_pairs(vec![0]),
            vec![0, 1]
        );
    }
}
