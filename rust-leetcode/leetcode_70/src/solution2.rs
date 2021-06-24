use std::collections::HashMap;

impl Solution {
  pub fn climb_stairs(n: i32) -> i32 {
    let mut m: HashMap<i32, i32> = HashMap::new();
    Solution::recursion(n, &mut m)
  }

  fn recursion(n: i32, m: &mut HashMap<i32, i32>) -> i32 {
    if n < 3 {
      return n;
    }
    if !m.contains_key(&n) {
      let val = Solution::recursion(n - 1, m) + Solution::recursion(n - 2, m);
      m.insert(n, val);
    }
    *m.get(&n).unwrap()
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
