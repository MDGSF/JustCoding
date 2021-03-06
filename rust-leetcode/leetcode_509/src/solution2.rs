// 简单递归 + 记忆化，时间复杂度 O(n)
use std::collections::HashMap;
impl Solution {
  pub fn fib(n: i32) -> i32 {
    let mut m = HashMap::new();
    m.insert(0, 0);
    m.insert(1, 1);
    Solution::fib_inner(n, m)
  }

  fn fib_inner(n: i32, mut m: HashMap<i32, i32>) -> i32 {
    if let Some(&v) = m.get(&n) {
      v
    } else {
      let v = Solution::fib(n - 1) + Solution::fib(n - 2);
      m.insert(n, v);
      v
    }
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
