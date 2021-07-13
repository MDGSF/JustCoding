use std::collections::HashMap;

impl Solution {
  pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
      return false;
    }

    let mut map = HashMap::new();

    for c in s.chars() {
      *map.entry(c).or_insert(0) += 1;
    }

    for c in t.chars() {
      let count = map.entry(c).or_insert(0);
      *count -= 1;

      // 如果计数器低于0，表示t包含了一个不在s的额外字母
      if *count < 0 {
        return false;
      }
    }

    true
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(
      Solution::is_anagram("anagram".to_string(), "nagaram".to_string()),
      true
    );
    assert_eq!(
      Solution::is_anagram("rat".to_string(), "car".to_string()),
      false
    );
  }
}
