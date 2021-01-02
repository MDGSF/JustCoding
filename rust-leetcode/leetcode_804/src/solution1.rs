use std::collections::HashSet;

impl Solution {
  pub fn unique_morse_representations(words: Vec<String>) -> i32 {
    let mut result = HashSet::new();
    for word in words.iter() {
      let morse_code = Solution::word_to_morse_code(word);
      result.insert(morse_code);
    }
    result.len() as i32
  }

  fn word_to_morse_code(word: &str) -> String {
    let morse_code = [
      ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--",
      "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--", "--..",
    ];
    let mut result = String::new();
    let word = word.as_bytes();
    for &c in word {
      result.push_str(morse_code[(c - b'a') as usize]);
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
    let words = vec![
      "gin".to_string(),
      "zen".to_string(),
      "gig".to_string(),
      "msg".to_string(),
    ];
    assert_eq!(Solution::unique_morse_representations(words), 2);
  }
}
