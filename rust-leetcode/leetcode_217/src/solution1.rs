use std::collections::HashMap;

impl Solution {
  pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut nums_counter: HashMap<i32, i32> = HashMap::new();
    for &num in nums.iter() {
      let v = nums_counter.entry(num).or_insert(0);
      if *v == 0 {
        *v += 1;
      } else {
        return true;
      }
    }
    false
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 1]), true);
    assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 4]), false);
    assert_eq!(
      Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]),
      true
    );
  }
}
