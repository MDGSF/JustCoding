// 题目 0 <= n <= 30，可以使用查表发，时间复杂度 O(1)
impl Solution {
  pub fn fib(n: i32) -> i32 {
    let m = [
      0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765,
      10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040,
    ];
    m[n as usize]
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
