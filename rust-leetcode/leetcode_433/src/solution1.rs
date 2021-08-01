use std::collections::HashSet;
use std::collections::VecDeque;
use std::iter::FromIterator;

impl Solution {
  pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
    if start == end {
      return 0;
    }
    let charset = vec!['A', 'C', 'G', 'T'];
    let bankset: HashSet<String> = HashSet::from_iter(bank.iter().cloned());
    let mut level = 0;
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back(start.clone());
    visited.insert(start.clone());
    while !queue.is_empty() {
      let levelsize = queue.len();
      for _i in 0..levelsize {
        let cur = queue.pop_front().unwrap();
        if cur == end {
          return level;
        }
        let mut cur_charset: Vec<char> = cur.chars().collect();
        for j in 0..cur_charset.len() {
          let old = cur_charset[j];
          for k in 0..charset.len() {
            cur_charset[j] = charset[k];
            let next: String = cur_charset.iter().collect();
            if !visited.contains(&next) && bankset.contains(&next) {
              visited.insert(next.clone());
              queue.push_back(next.clone());
            }
          }
          cur_charset[j] = old;
        }
      }
      level += 1;
    }
    -1
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(
      Solution::min_mutation(
        "AACCGGTT".to_string(),
        "AACCGGTA".to_string(),
        vec!["AACCGGTA".to_string()]
      ),
      1
    );
  }
}
