impl Solution {
  pub fn climb_stairs(n: i32) -> i32 {
    if n < 3 {
      return n;
    }
    let mut f1 = 1;
    let mut f2 = 2;
    for _ in 2..n {
      let next = f1 + f2;
      f1 = f2;
      f2 = next;
    }
    return f2;
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
