use std::collections::HashMap;

impl Solution {
  pub fn num_rabbits(answers: Vec<i32>) -> i32 {
    let mut count: HashMap<usize, usize> = HashMap::new();
    for answer in answers {
      *count.entry(answer as usize).or_insert(0) += 1;
    }
    let mut result = 0;
    for (y, &x) in count.iter() {
      // x 只兔子回答了 y
      result += (x + y) / (y + 1) * (y + 1);
    }
    result as i32
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    assert_eq!(Solution::num_rabbits(vec![1, 1, 2]), 5);
    assert_eq!(Solution::num_rabbits(vec![10, 10, 10]), 11);
    assert_eq!(Solution::num_rabbits(vec![]), 0);
  }
}
