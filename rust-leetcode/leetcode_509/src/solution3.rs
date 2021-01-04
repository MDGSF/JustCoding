// 简单递推，时间复杂度 O(n)
impl Solution {
  pub fn fib(n: i32) -> i32 {
    let mut f1 = 0;
    let mut f2 = 1;
    for _i in 0..n {
      let t = f1 + f2;
      f1 = f2;
      f2 = t;
    }
    f1
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
