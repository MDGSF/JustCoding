use std::collections::HashSet;
use std::collections::VecDeque;
use std::iter::FromIterator;

impl Solution {
  pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
    let char_set: Vec<char> = ('a'..='z').collect::<Vec<char>>();
    let word_set: HashSet<String> = HashSet::from_iter(word_list.iter().cloned());
    let mut visited: HashSet<String> = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(begin_word);
    let mut level = 0;
    while !queue.is_empty() {
      let levelsize = queue.len();
      for _i in 0..levelsize {
        let cur = queue.pop_front().unwrap();
        if cur == end_word {
          return level + 1;
        }
        let mut cur_charset: Vec<char> = cur.chars().collect();
        for j in 0..cur_charset.len() {
          let old = cur_charset[j];
          for k in 0..char_set.len() {
            cur_charset[j] = char_set[k];
            let next: String = cur_charset.iter().collect();
            if !visited.contains(&next) && word_set.contains(&next) {
              visited.insert(next.clone());
              queue.push_back(next.clone());
            }
          }
          cur_charset[j] = old;
        }
      }
      level += 1;
    }
    0
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(
      Solution::ladder_length(
        "hit".to_string(),
        "cog".to_string(),
        vec![
          "hot".to_string(),
          "dot".to_string(),
          "dog".to_string(),
          "lot".to_string(),
          "log".to_string(),
          "cog".to_string()
        ]
      ),
      5
    );
  }
}
