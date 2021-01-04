// 时间复杂度 O(1)，通项公式
// return Math.round(
//   (
//     Math.pow((1 + Math.sqrt(5))/2 , N)
//     -
//     Math.pow((1 - Math.sqrt(5))/2, N)
//   )
//   /
//   Math.sqrt(5)
// );
impl Solution {
  pub fn fib(n: i32) -> i32 {
    ((((1_f64 + 5_f64.sqrt()) / 2_f64).powf(n as f64)
      - ((1_f64 - 5_f64.sqrt()) / 2_f64).powf(n as f64))
      / 5_f64.sqrt())
    .round() as i32
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
