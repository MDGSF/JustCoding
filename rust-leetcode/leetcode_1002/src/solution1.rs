use std::collections::HashMap;

impl Solution {
  pub fn common_chars(a: Vec<String>) -> Vec<String> {
    let mut m1 = Solution::counter(&a[0]);
    for i in 1..a.len() {
      let m2 = Solution::counter(&a[i]);
      m1 = Solution::intersection(m1, m2);
    }
    let mut result = Vec::new();
    for (c, n) in m1 {
      for _ in 0..n {
        result.push(c.to_string());
      }
    }
    result
  }

  fn counter(s: &str) -> HashMap<char, i32> {
    let mut result = HashMap::new();
    for c in s.chars() {
      *result.entry(c).or_insert(0) += 1;
    }
    result
  }

  fn intersection(m1: HashMap<char, i32>, m2: HashMap<char, i32>) -> HashMap<char, i32> {
    let mut result = HashMap::new();
    for (c1, n1) in m1 {
      if let Some(&n2) = m2.get(&c1) {
        result.insert(c1, n1.min(n2));
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
      Solution::common_chars(vec![
        "bella".to_string(),
        "label".to_string(),
        "roller".to_string(),
      ])
      .sort(),
      vec!["e".to_string(), "l".to_string(), "l".to_string()].sort()
    );

    assert_eq!(
      Solution::common_chars(vec![
        "cool".to_string(),
        "lock".to_string(),
        "cook".to_string(),
      ])
      .sort(),
      vec!["c".to_string(), "o".to_string()].sort()
    );
  }
}
