// 11 / 3
// dividend = 11
// divisor = 3
//
// 11 > 3, result >= 1
// 3 * 2 = 6, 11 > 6, result >= 2
// 6 * 2 = 12, 11 < 12, 2 <= result < 4

impl Solution {
  pub fn divide(dividend: i32, divisor: i32) -> i32 {
    if dividend == 0 {
      return 0;
    }
    if divisor == 1 {
      return dividend;
    }
    if divisor == -1 {
      if dividend > i32::MIN {
        return -dividend;
      }
      return i32::MAX;
    }
    let a: i64 = dividend as i64;
    let b: i64 = divisor as i64;
    let sign = if (a > 0 && b < 0) || (a < 0 && b > 0) {
      -1
    } else {
      1
    };
    let a = if a > 0 { a } else { -a };
    let b = if b > 0 { b } else { -b };
    let result = Solution::div(a, b);
    if sign > 0 {
      if result > i32::MAX {
        return i32::MAX;
      } else {
        return result as i32;
      }
    }
    -result
  }

  fn div(a: i64, b: i64) -> i32 {
    if a < b {
      return 0;
    }
    let mut count = 1;
    let mut tempb = b;
    while (tempb + tempb) <= a {
      count = count + count;
      tempb = tempb + tempb;
    }
    count + Solution::div(a - tempb, b)
  }
}

struct Solution;

fn main() {
  let dividend = 7;
  let divisor = -3;
  let result = Solution::divide(dividend, divisor);
  println!("result = {}", result);
}
