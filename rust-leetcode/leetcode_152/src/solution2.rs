// dp[i][2]
// dp[i][0] 表示从 0->i（包括第i个节点）的正的最大值。
// dp[i][1] 表示从 0->i（包括第i个节点）的负的最大值，也就是最小值。
//
// dp[i][0] =
//   if (nums[i] >= 0) dp[i-1][0] * nums[i]
//   else dp[i-1][1] * nums[i]
//
// dp[i][1] =
//   if (nums[i] >= 0) dp[i-1][1] * nums[i]
//   else dp[i-1][0] * nums[1]
//
// result = max(dp[0][0], dp[1][0], ..., dp[i-1][0], dp[i][0])
impl Solution {
  pub fn max_product(nums: Vec<i32>) -> i32 {
    let (mut result, mut pre_max, mut pre_min) = (nums[0], nums[0], nums[0]);
    for i in 1..nums.len() {
      let (t_max, t_min) = (pre_max, pre_min);
      pre_max = nums[i].max(t_max * nums[i]).max(t_min * nums[i]);
      pre_min = nums[i].min(t_max * nums[i]).min(t_min * nums[i]);
      result = result.max(pre_max);
    }
    result
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(Solution::max_product(vec![2, 3, -2, 4]), 6);
    assert_eq!(Solution::max_product(vec![-2, 0, -1]), 0);
    assert_eq!(Solution::max_product(vec![-4, -3, -2]), 12);
  }
}
