impl Solution {
    pub fn left_rigth_difference(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();

        let mut presum = vec![0; n];
        for i in 1..n {
            presum[i] = presum[i - 1] + nums[i - 1];
        }

        let mut suffixsum = vec![0; n];
        for i in (0..(n - 1)).rev() {
            suffixsum[i] = suffixsum[i + 1] + nums[i + 1];
        }

        let mut result = vec![0; n];
        for i in 0..n {
            result[i] = (presum[i] - suffixsum[i]).abs();
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
            Solution::left_rigth_difference(vec![10, 4, 8, 3]),
            vec![15, 1, 11, 22]
        );
    }

    #[test]
    fn test02() {
        assert_eq!(Solution::left_rigth_difference(vec![1]), vec![0]);
    }
}
