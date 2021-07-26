impl Solution {
  pub fn my_pow(x: f64, n: i32) -> f64 {
    Solution::my_pow_inner(x, n as i64)
  }

  fn my_pow_inner(mut x: f64, mut n: i64) -> f64 {
    if n < 0 {
      x = 1f64 / x;
      n = -n;
    }
    let mut pow = 1f64;
    while n > 0 {
      if (n & 1) == 1 {
        pow *= x;
      }
      x *= x;
      n >>= 1;
    }
    pow
  }
}

pub struct Solution;
