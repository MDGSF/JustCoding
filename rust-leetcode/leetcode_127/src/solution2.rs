use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
  pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
    let word_set: HashSet<String> = HashSet::from_iter(word_list.iter().cloned());
    if !word_set.contains(&end_word) {
      return 0;
    }
    let char_set: Vec<char> = ('a'..='z').collect::<Vec<char>>();
    let mut visited: HashSet<String> = HashSet::new();
    let mut level = 1;
    let mut begin_set = HashSet::new();
    let mut end_set = HashSet::new();
    begin_set.insert(begin_word);
    end_set.insert(end_word);

    while !begin_set.is_empty() && !end_set.is_empty() {
      if begin_set.len() > end_set.len() {
        std::mem::swap(&mut begin_set, &mut end_set);
      }
      let mut temp = HashSet::new();
      for word in begin_set.into_iter() {
        let mut cur_charset: Vec<char> = word.chars().collect();
        for j in 0..cur_charset.len() {
          let old = cur_charset[j];
          for k in 0..char_set.len() {
            cur_charset[j] = char_set[k];
            let next: String = cur_charset.iter().collect();
            if end_set.contains(&next) {
              return level + 1;
            }

            if !visited.contains(&next) && word_set.contains(&next) {
              visited.insert(next.clone());
              temp.insert(next.clone());
            }
          }
          cur_charset[j] = old;
        }
      }
      begin_set = temp;
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
