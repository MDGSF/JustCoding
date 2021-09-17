use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Default, Debug)]
struct Trie {
  root: HashMap<char, Trie>,
  is_word: bool,
}

impl Trie {
  fn new() -> Self {
    Self::default()
  }

  fn insert(&mut self, word: String) {
    let mut cur = self;
    for c in word.chars() {
      cur = cur.root.entry(c).or_insert(Trie::new());
    }
    cur.is_word = true;
  }
}

impl Solution {
  pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
    let mut trie = Trie::new();
    for word in words.iter() {
      trie.insert(word.to_string());
    }

    let rows = board.len();
    let cols = board[0].len();
    let mut visited = vec![vec![false; cols]; rows];

    let mut result: HashSet<String> = HashSet::new();
    for row in 0..rows {
      for col in 0..cols {
        let c = board[row][col];
        if let Some(sub_trie) = trie.root.get(&c) {
          let mut cur_str = String::new();
          cur_str.push(c);
          Self::dfs(
            &sub_trie,
            &board,
            row,
            col,
            cur_str,
            &mut result,
            &mut visited,
          );
        }
      }
    }

    result.into_iter().collect()
  }

  fn dfs(
    trie: &Trie,
    board: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    cur_str: String,
    result: &mut HashSet<String>,
    visited: &mut Vec<Vec<bool>>,
  ) {
    if trie.is_word {
      result.insert(cur_str.clone());
    }
    if !Self::can_walk(board, row as isize, col as isize, visited) {
      return;
    }

    let c = board[row][col];
    if !trie.root.contains_key(&c) {
      return;
    }

    visited[row][col] = true;

    let directions = vec![vec![-1, 0], vec![1, 0], vec![0, -1], vec![0, 1]];
    let old_row = row as isize;
    let old_col = col as isize;
    for direction in directions.iter() {
      let new_row = old_row + direction[0];
      let new_col = old_col + direction[1];
      if Self::can_walk(board, new_row, new_col, visited) {
        if let Some(sub_trie) = trie.root.get(&c) {
          let mut new_str = cur_str.clone();
          new_str.push(c);
          Self::dfs(
            sub_trie,
            &board,
            new_row as usize,
            new_col as usize,
            new_str,
            result,
            visited,
          );
        }
      }
    }

    visited[row][col] = false;
  }

  fn can_walk(
    board: &Vec<Vec<char>>,
    row: isize,
    col: isize,
    visited: &mut Vec<Vec<bool>>,
  ) -> bool {
    0 <= row
      && (row as usize) < board.len()
      && 0 <= col
      && (col as usize) < board[0].len()
      && !visited[row as usize][col as usize]
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let board = vec![vec!['a'; 1]; 1];
    let words = vec!["a".to_string()];
    assert_eq!(Solution::find_words(board, words), vec!["a".to_string()]);
  }
}
