use std::collections::HashMap;
use std::str::Chars;

impl Solution {
  pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
      return Vec::new();
    }

    let mut m: HashMap<char, Vec<char>> = HashMap::new();
    m.insert('2', vec!['a', 'b', 'c']);
    m.insert('3', vec!['d', 'e', 'f']);
    m.insert('4', vec!['g', 'h', 'i']);
    m.insert('5', vec!['j', 'k', 'l']);
    m.insert('6', vec!['m', 'n', 'o']);
    m.insert('7', vec!['p', 'q', 'r', 's']);
    m.insert('8', vec!['t', 'u', 'v']);
    m.insert('9', vec!['w', 'x', 'y', 'z']);

    let mut chars = digits.chars();
    let mut node = String::new();
    let mut result = Vec::new();
    Self::recursion(&m, &mut chars, &mut node, &mut result);
    result
  }

  fn recursion(
    m: &HashMap<char, Vec<char>>,
    chars: &mut Chars,
    node: &mut String,
    result: &mut Vec<String>,
  ) {
    match chars.next() {
      Some(c) => {
        let letters = m.get(&c).unwrap();
        for &letter in letters.iter() {
          node.push(letter);
          Self::recursion(m, &mut chars.clone(), node, result);
          node.pop();
        }
      }
      None => {
        result.push(node.clone());
        return;
      }
    }
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {}
}
