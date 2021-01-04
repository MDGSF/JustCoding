// 时间复杂度 O(n) 的递归
impl Solution {
  pub fn fib(n: i32) -> i32 {
    Solution::fib_inner(n, 0, 0, 1)
  }

  fn fib_inner(n: i32, i: i32, f1: i32, f2: i32) -> i32 {
    if i == n {
      return f1;
    }
    Solution::fib_inner(n, i + 1, f2, f1 + f2)
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
