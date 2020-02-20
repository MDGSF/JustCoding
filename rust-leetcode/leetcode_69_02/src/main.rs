impl Solution {
  pub fn my_sqrt(x: i32) -> i32 {
    return Solution::my_inner_sqrt(x as f64, 0.01f64) as i32;
  }

  fn my_inner_sqrt(x: f64, precision: f64) -> f64 {
    if x <= 1f64 {
      return x;
    }
    let mut left = 0f64;
    let mut right = x;
    while left < right {
      let mid = left + (right - left) / 2f64;
      let cur = mid * mid;
      if (cur - x).abs() <= precision {
        return mid;
      } else if cur > x {
        right = mid;
      } else {
        left = mid;
      }
    }
    0.0
  }
}

struct Solution {}

fn main() {
  let result = Solution::my_sqrt(5);
  println!("result = {}", result);
}
