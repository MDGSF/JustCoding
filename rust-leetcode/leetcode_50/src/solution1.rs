impl Solution {
  pub fn my_pow(mut x: f64, mut n: i32) -> f64 {
    if n < 0 {
      x = 1_f64 / x;
      n = -n;
    }

    let mut result = 1_f64;
    for _i in 0..n {
      result *= x;
    }
    result
  }
}

pub struct Solution;
