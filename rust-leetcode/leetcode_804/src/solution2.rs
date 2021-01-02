impl Solution {
  pub fn unique_morse_representations(words: Vec<String>) -> i32 {
    let morse_code = [
      ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--",
      "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--", "--..",
    ];
    words
      .iter()
      .map(|word| {
        word
          .bytes()
          .flat_map(|c| morse_code[(c - b'a') as usize].chars())
          .collect()
      })
      .collect::<std::collections::HashSet<String>>()
      .len() as i32
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
