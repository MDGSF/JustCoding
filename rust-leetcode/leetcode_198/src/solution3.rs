impl Solution {
  pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
      return 0;
    } else if nums.len() == 1 {
      return nums[0];
    } else if nums.len() == 2 {
      return nums[0].max(nums[1]);
    }
    let mut dp = vec![0; nums.len()];
    dp[0] = nums[0];
    dp[1] = nums[0].max(nums[1]);
    for i in 2..nums.len() {
      dp[i] = dp[i - 1].max(dp[i - 2] + nums[i]);
    }
    dp[nums.len() - 1]
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
  }
}
