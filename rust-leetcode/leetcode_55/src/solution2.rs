impl Solution {
  pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut reach = 0;
    let mut i = 0;
    while i <= reach && reach < nums.len() {
      reach = reach.max(i + nums[i] as usize);
      i += 1;
    }
    reach >= nums.len() - 1
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
    assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
  }
}
