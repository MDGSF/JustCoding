impl Solution {
  pub fn is_anagram(s: String, t: String) -> bool {
    Solution::calc_key(s) == Solution::calc_key(t)
  }

  fn calc_key(onestr: String) -> String {
    let mut chars: Vec<char> = onestr.chars().collect();
    chars.sort();
    chars.into_iter().collect()
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(
      Solution::is_anagram("anagram".to_string(), "nagaram".to_string()),
      true
    );
    assert_eq!(
      Solution::is_anagram("rat".to_string(), "car".to_string()),
      false
    );
  }
}
