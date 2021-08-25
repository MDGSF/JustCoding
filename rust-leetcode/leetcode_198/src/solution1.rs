impl Solution {
  pub fn rob(nums: Vec<i32>) -> i32 {
    let (mut f1, mut f2) = (0, 0);
    for &num in nums.iter() {
      let (t_f1, t_f2) = (f1, f2);
      f1 = t_f2;
      f2 = t_f2.max(t_f1 + num);
    }
    f2
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
