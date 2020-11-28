impl Solution {
  pub fn reverse(x: i32) -> i32 {
    let sign = if x.is_negative() { -1 } else { 1 };
    let mut x = x.abs();
    let mut result = 0i64;
    while x > 0 {
      let bit = x % 10;
      result = result * 10 + bit as i64;
      x /= 10;
    }
    result *= sign;
    if result > i32::MAX as i64 || result < i32::MIN as i64 {
      0
    } else {
      result as i32
    }
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    assert_eq!(Solution::reverse(123), 321);
    assert_eq!(Solution::reverse(-123), -321);
    assert_eq!(Solution::reverse(120), 21);
  }
}
