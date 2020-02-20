impl Solution {
  pub fn my_sqrt(x: i32) -> i32 {
    if x <= 1 {
      return x;
    }
    let mut r: i64 = x as i64;
    while r * r > x as i64 {
      r = (r + x as i64 / r) / 2i64;
    }
    return r as i32;
  }
}

struct Solution {}

fn main() {
  let result = Solution::my_sqrt(5);
  println!("result = {}", result);
}
