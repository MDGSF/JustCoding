impl Solution {
  pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
      return 0;
    } else if nums.len() == 1 {
      return nums[0];
    }
    let sub1 = Self::rob_198(&nums[1..]);
    let sub2 = Self::rob_198(&nums[..nums.len() - 1]);
    sub1.max(sub2)
  }

  pub fn rob_198(nums: &[i32]) -> i32 {
    nums
      .iter()
      .fold((0, 0), |(f1, f2), &num| (f2, f2.max(f1 + num)))
      .1
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(Solution::rob(vec![2, 3, 2]), 3);
    assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    assert_eq!(Solution::rob(vec![0]), 0);
  }
}
