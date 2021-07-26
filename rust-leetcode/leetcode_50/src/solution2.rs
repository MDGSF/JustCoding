impl Solution {
  pub fn my_pow(x: f64, n: i32) -> f64 {
    match n {
      n if n == -2147483648 => 1.0 / Solution::my_pow(x * x, -(n / 2)),
      n if n < 0 => 1f64 / Solution::my_pow(x, -n),
      n if n == 0 => 1f64,
      n if n % 2 == 0 => Solution::my_pow(x * x, n / 2),
      _ => x * Solution::my_pow(x, n - 1),
    }
  }
}

pub struct Solution;
