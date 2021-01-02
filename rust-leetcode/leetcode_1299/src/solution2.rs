impl Solution {
  pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
    let mut max_num = i32::MIN;
    let mut result = vec![-1; arr.len()];
    for i in (0..(arr.len() - 1)).rev() {
      max_num = max_num.max(arr[i + 1]);
      result[i] = max_num;
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
      Solution::replace_elements(vec![17, 18, 5, 4, 6, 1]),
      vec![18, 6, 6, 6, 1, -1]
    );
  }
}
