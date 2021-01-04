// 简单递推，函数式写法，时间复杂度 O(n)
impl Solution {
  pub fn fib(n: i32) -> i32 {
    (0..n).fold((0u64, 1), |(p1, p2), _| (p2, p1 + p2)).0 as i32
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
