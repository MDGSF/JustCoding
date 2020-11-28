impl Solution {
  pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
    for i in 0..nums.len() {
      while nums[i] > 0
        && (nums[i] as usize) < nums.len()
        && nums[i] as usize != i + 1
        && nums[nums[i] as usize - 1] != nums[i]
      {
        let num = nums[i];
        nums.swap(i, num as usize - 1);
      }
    }

    for i in 0..nums.len() {
      if nums[i] != (i + 1) as i32 {
        return (i + 1) as i32;
      }
    }

    (nums.len() + 1) as i32
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    assert_eq!(Solution::first_missing_positive(vec![]), 1);
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 0]), 3);
    assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
    assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
    assert_eq!(Solution::first_missing_positive(vec![1, 1]), 2);
  }
}
