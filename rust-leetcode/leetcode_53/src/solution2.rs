impl Solution {
  pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut cur = nums[0];
    let mut result = cur;
    for i in 1..nums.len() {
      cur = std::cmp::max(cur + nums[i], nums[i]);
      result = std::cmp::max(result, cur);
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
    assert_eq!(
      Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
      6
    );
    assert_eq!(Solution::max_sub_array(vec![1]), 1);
    assert_eq!(Solution::max_sub_array(vec![0]), 0);
    assert_eq!(Solution::max_sub_array(vec![-1]), -1);
    assert_eq!(Solution::max_sub_array(vec![-10000]), -10000);
  }
}
