use std::collections::HashSet;
use std::collections::VecDeque;
use std::iter::FromIterator;

impl Solution {
  pub fn find_ladders(
    begin_word: String,
    end_word: String,
    word_list: Vec<String>,
  ) -> Vec<Vec<String>> {
    let char_set: Vec<char> = ('a'..='z').collect::<Vec<char>>();
    let word_set: HashSet<String> = HashSet::from_iter(word_list.iter().cloned());
    let mut result = Vec::new();
    let mut visited: HashSet<String> = HashSet::new();
    let mut queue: VecDeque<Vec<String>> = VecDeque::new();
    queue.push_back(vec![begin_word]);
    let mut found_result = false;
    while !queue.is_empty() && !found_result {
      let levelsize = queue.len();
      let mut sub_visited = HashSet::new();
      for _i in 0..levelsize {
        let path = queue.pop_front().unwrap();
        let cur = path.last().unwrap().to_string();
        let mut cur_charset: Vec<char> = cur.chars().collect();
        for j in 0..cur_charset.len() {
          let old = cur_charset[j];
          for k in 0..char_set.len() {
            cur_charset[j] = char_set[k];
            let next: String = cur_charset.iter().collect();
            if !visited.contains(&next) && word_set.contains(&next) {
              let mut newpath = path.clone();
              newpath.push(next.clone());
              if next == end_word {
                found_result = true;
                result.push(newpath.clone());
              }
              queue.push_back(newpath);
              sub_visited.insert(next.clone());
            }
          }
          cur_charset[j] = old;
        }
      }
      sub_visited.into_iter().for_each(|elem| {
        visited.insert(elem);
      });
    }
    result
  }
}

pub struct Solution;
