impl Solution {
  pub fn jump(nums: Vec<i32>) -> i32 {
    let mut end = 0;
    let mut max_position = 0;
    let mut steps = 0;
    for i in 0..(nums.len() - 1) {
      max_position = max_position.max(i + nums[i] as usize);
      if i == end {
        end = max_position;
        steps += 1;
      }
    }
    steps
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
    assert_eq!(Solution::jump(vec![2, 3, 0, 1, 4]), 2);
  }
}
