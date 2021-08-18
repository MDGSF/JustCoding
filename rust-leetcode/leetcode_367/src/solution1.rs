impl Solution {
  pub fn is_perfect_square(num: i32) -> bool {
    let num: i64 = num as i64;
    if num < 2 {
      return true;
    }
    let mut left = 2;
    let mut right = num / 2;
    while left <= right {
      let mid = left + (right - left) / 2;
      let guess_square = mid * mid;
      if guess_square == num {
        return true;
      } else if guess_square < num {
        left = mid + 1;
      } else {
        right = mid - 1;
      }
    }
    false
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert!(Solution::is_perfect_square(16));
    assert!(!Solution::is_perfect_square(14));
    assert!(Solution::is_perfect_square(808201));
  }
}
