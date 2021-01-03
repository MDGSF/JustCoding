use std::collections::HashMap;

impl Solution {
  pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
    let mut m = HashMap::new();
    for cpdomain in cpdomains.iter() {
      let space_idx = cpdomain.find(' ').unwrap();
      let num: i32 = cpdomain[..space_idx].parse().unwrap();
      *m.entry(&cpdomain[space_idx + 1..]).or_insert(0) += num;
      let mut start = 0;
      while let Some(idx) = cpdomain[space_idx + 1..][start..].find('.') {
        start += idx + 1;
        let key = &cpdomain[space_idx + 1..][start..];
        *m.entry(key).or_insert(0) += num;
      }
    }

    let mut result = Vec::new();
    for (key, val) in &m {
      result.push(format!("{} {}", val, key));
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
      Solution::subdomain_visits(vec!["9001 discuss.leetcode.com".to_string()]).sort(),
      vec![
        "9001 discuss.leetcode.com".to_string(),
        "9001 leetcode.com".to_string(),
        "9001 com".to_string()
      ]
      .sort()
    );
  }
}
