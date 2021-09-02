// dp[i] 表示以下标 i 结尾的最长有效子字符串的长度
//
// s[i] == ')' && s[i-1] == '('
//   dp[i] = dp[i-2] + 2
//
// s[i] == ')' && s[i-1] == ')' && s[i - dp[i-1] - 1] == '('
//   dp[i] = dp[i-1] + dp[i - dp[i-1] - 2] + 2
//
// result = max(dp[0], dp[1], ..., dp[i])
impl Solution {
  pub fn longest_valid_parentheses(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let mut result = 0;
    let mut dp = vec![0; s.len()];
    for (i, _c) in chars.iter().enumerate().skip(1).filter(|(_i, &c)| c == ')') {
      if chars[i - 1] == '(' {
        if (i as i32 - 2) >= 0 {
          dp[i] = dp[i - 2] + 2;
        } else {
          dp[i] = 2;
        }
      } else if (i as i32 - dp[i - 1] as i32 - 1) >= 0 && chars[i - dp[i - 1] - 1] == '(' {
        if (i as i32 - dp[i - 1] as i32 - 2) >= 0 {
          dp[i] = dp[i - 1] + dp[i - dp[i - 1] - 2] + 2;
        } else {
          dp[i] = dp[i - 1] + 2;
        }
      }

      result = result.max(dp[i]);
    }
    result as i32
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(Solution::longest_valid_parentheses("(()".to_string()), 2);
    assert_eq!(Solution::longest_valid_parentheses(")()())".to_string()), 4);
    assert_eq!(Solution::longest_valid_parentheses("".to_string()), 0);
  }
}
