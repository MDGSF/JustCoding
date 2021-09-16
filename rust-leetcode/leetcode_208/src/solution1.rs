/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */
use std::collections::HashMap;

#[derive(Default)]
struct Trie {
  root: HashMap<char, Trie>,
  is_word: bool,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
  /** Initialize your data structure here. */
  fn new() -> Self {
    Self::default()
  }

  /** Inserts a word into the trie. */
  fn insert(&mut self, word: String) {
    let mut cur = self;
    for c in word.chars() {
      cur = cur.root.entry(c).or_insert(Trie::new())
    }
    cur.is_word = true;
  }

  /** Returns if the word is in the trie. */
  fn search(&self, word: String) -> bool {
    let mut cur = self;
    for c in word.chars() {
      match cur.root.get(&c) {
        Some(node) => cur = node,
        None => return false,
      }
    }
    cur.is_word
  }

  /** Returns if there is any word in the trie that starts with the given prefix. */
  fn starts_with(&self, prefix: String) -> bool {
    let mut cur = self;
    for c in prefix.chars() {
      match cur.root.get(&c) {
        Some(node) => cur = node,
        None => return false,
      }
    }
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let mut trie = Trie::new();
    trie.insert("apple".to_string());
    assert_eq!(trie.search("apple".to_string()), true);
    assert_eq!(trie.search("app".to_string()), false);
    assert_eq!(trie.starts_with("app".to_string()), true);
    trie.insert("app".to_string());
    assert_eq!(trie.search("apple".to_string()), true);
  }
}
