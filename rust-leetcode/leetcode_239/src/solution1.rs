impl Solution {
  pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    if nums.is_empty() {
      return vec![];
    }
    let k = k as usize;
    let mut result = Vec::new();
    let mut window = Vec::new();
    let length = nums.len();
    for i in 0..length {
      if i >= k && window[0] <= i - k {
        window.remove(0);
      }
      while !window.is_empty() && nums[window[window.len() - 1]] <= nums[i] {
        window.pop();
      }
      window.push(i);
      if i >= k - 1 {
        result.push(nums[window[0]]);
      }
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
      Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
      vec![3, 3, 5, 5, 6, 7]
    );

    assert_eq!(Solution::max_sliding_window(vec![1], 1), vec![1]);

    assert_eq!(Solution::max_sliding_window(vec![1, -1], 1), vec![1, -1]);

    assert_eq!(Solution::max_sliding_window(vec![9, 11], 2), vec![11]);

    assert_eq!(Solution::max_sliding_window(vec![4, -2], 2), vec![4]);
  }
}
