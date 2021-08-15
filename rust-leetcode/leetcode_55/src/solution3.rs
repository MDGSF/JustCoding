impl Solution {
  pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut last = nums.len() - 1;
    for i in (0..nums.len()).rev() {
      let reach = i + nums[i] as usize;
      if reach >= last {
        last = i;
      }
    }
    last == 0
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
