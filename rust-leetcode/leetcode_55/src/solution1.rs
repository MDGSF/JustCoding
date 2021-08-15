impl Solution {
  pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut reach = 0;
    for (i, &num) in nums.iter().enumerate() {
      if i > reach {
        return false;
      }
      reach = reach.max(i + num as usize);
    }
    true
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
