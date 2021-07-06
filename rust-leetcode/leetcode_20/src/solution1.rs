impl Solution {
  pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for c in s.chars() {
      if c == '(' {
        stack.push(')');
      } else if c == '[' {
        stack.push(']');
      } else if c == '{' {
        stack.push('}');
      } else {
        if stack.is_empty() || c != stack.pop().unwrap() {
          return false;
        }
      }
    }
    stack.is_empty()
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(Solution::is_valid("".to_string()), true);
    assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
    assert_eq!(Solution::is_valid("{[]}".to_string()), true);
    assert_eq!(Solution::is_valid("([)]".to_string()), false);
  }
}
