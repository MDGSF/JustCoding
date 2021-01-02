impl Solution {
  pub fn rotated_digits(n: i32) -> i32 {
    let mut count = 0;
    for i in 1..=n {
      if Solution::is_good_number(i) {
        count += 1;
      }
    }
    count
  }

  fn is_good_number(mut n: i32) -> bool {
    let mut has_rotate_diff_number = false;
    while n > 0 {
      let t = n % 10;
      n = n / 10;
      if !Solution::is_rotate_number(t) {
        return false;
      }
      if Solution::is_rotate_diff_number(t) {
        has_rotate_diff_number = true;
      }
    }
    has_rotate_diff_number
  }

  fn is_rotate_number(n: i32) -> bool {
    n == 0 || n == 1 || n == 2 || n == 5 || n == 6 || n == 8 || n == 9
  }

  fn is_rotate_diff_number(n: i32) -> bool {
    n == 2 || n == 5 || n == 6 || n == 9
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    assert_eq!(Solution::rotated_digits(10), 4);
  }
}
