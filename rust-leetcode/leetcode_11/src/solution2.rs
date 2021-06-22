impl Solution {
  pub fn max_area(height: Vec<i32>) -> i32 {
    let mut result = 0;
    for i in 0..height.len() {
      for j in i+1..height.len() {
        let width = (j - i) as i32;
        let height = std::cmp::min(height[i], height[j]);
        let area = width * height;
        if area > result {
          result = area;
        }
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
    assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
  }
}
