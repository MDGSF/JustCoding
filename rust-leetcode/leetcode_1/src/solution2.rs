impl Solution {
  pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let size = nums.len();
    for i in 0..size {
      for j in i + 1..size {
        if nums[i] + nums[j] == target {
          return vec![i as i32, j as i32];
        }
      }
    }
    vec![]
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
  }
}
