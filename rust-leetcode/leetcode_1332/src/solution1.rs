impl Solution {
  pub fn remove_palindrome_sub(s: String) -> i32 {
    if s.is_empty() {
      return 0;
    }
    let s = s.as_bytes();
    let mut start = 0;
    let mut end = s.len() - 1;
    while start < end {
      if s[start] != s[end] {
        return 2;
      }
      start += 1;
      end -= 1;
    }
    1
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    assert_eq!(Solution::remove_palindrome_sub("ababa".to_string()), 1);
    assert_eq!(Solution::remove_palindrome_sub("abb".to_string()), 2);
    assert_eq!(Solution::remove_palindrome_sub("baabb".to_string()), 2);
    assert_eq!(Solution::remove_palindrome_sub("".to_string()), 0);
  }
}
