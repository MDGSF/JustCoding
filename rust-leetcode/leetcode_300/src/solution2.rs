/*
dp[i]: 考虑前i个元素，以第i个数字结尾的最长上升子序列的长度，nums[i]必须被选取。
dp[i] = max(dp[j]) + 1, 0 <= j < i, nums[j] < nums[i]
result = max(dp[i]), 0 <= i < n
*/
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let n = nums.len();
        let mut dp = vec![1; n];
        let mut result = 1;
        for i in 1..n {
            for j in 0..i {
                if nums[j] < nums[i] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
            result = result.max(dp[i]);
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
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    }
}
