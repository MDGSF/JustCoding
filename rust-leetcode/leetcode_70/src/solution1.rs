impl Solution {
  pub fn climb_stairs(n: i32) -> i32 {
    if n < 3 {
      return n;
    }
    return Solution::climb_stairs(n - 1) + Solution::climb_stairs(n - 2);
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    assert_eq!(Solution::climb_stairs(1), 1);
    assert_eq!(Solution::climb_stairs(2), 2);
    assert_eq!(Solution::climb_stairs(3), 3);
    assert_eq!(Solution::climb_stairs(4), 5);
  }
}
