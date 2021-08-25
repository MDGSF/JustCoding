impl Solution {
  pub fn rob(nums: Vec<i32>) -> i32 {
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
    assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
  }
}
