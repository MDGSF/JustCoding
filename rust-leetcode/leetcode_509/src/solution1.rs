// 简单递归，时间复杂度 O(2^n)
impl Solution {
  pub fn fib(n: i32) -> i32 {
    if n < 2 {
      return n;
    }
    Solution::fib(n - 1) + Solution::fib(n - 2)
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    assert_eq!(Solution::fib(0), 0);
    assert_eq!(Solution::fib(1), 1);
    assert_eq!(Solution::fib(2), 1);
    assert_eq!(Solution::fib(3), 2);
    assert_eq!(Solution::fib(4), 3);
    assert_eq!(Solution::fib(5), 5);
    assert_eq!(Solution::fib(6), 8);
  }
}
