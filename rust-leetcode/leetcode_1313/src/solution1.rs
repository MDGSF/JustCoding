impl Solution {
  pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for i in (0..nums.len()).step_by(2) {
      (0..nums[i]).for_each(|_j| result.push(nums[i + 1]));
    }
    result
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    assert_eq!(
      Solution::decompress_rl_elist(vec![1, 2, 3, 4]),
      vec![2, 4, 4, 4]
    );

    assert_eq!(
      Solution::decompress_rl_elist(vec![1, 1, 2, 3]),
      vec![1, 3, 3]
    );
  }
}
