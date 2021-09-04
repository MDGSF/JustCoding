impl Solution {
  pub fn num_decodings(s: String) -> i32 {
    let s: Vec<char> = s.chars().collect();
    let mut dp = vec![0; s.len()];
    if s[0] != '0' {
      dp[0] = 1;
    }
    for i in 1..s.len() {
      let first = s[i].to_digit(10).unwrap();
      let second = s[i - 1].to_digit(10).unwrap() * 10 + s[i].to_digit(10).unwrap();
      if first >= 1 && first <= 9 {
        dp[i] += dp[i - 1];
      }
      if second >= 10 && second <= 26 {
        if i >= 2 {
          dp[i] += dp[i - 2];
        } else {
          dp[i] += 1;
        }
      }
    }
    dp[dp.len() - 1]
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(Solution::num_decodings("12".to_string()), 2);
    assert_eq!(Solution::num_decodings("226".to_string()), 3);
    assert_eq!(Solution::num_decodings("06".to_string()), 0);
  }
}
