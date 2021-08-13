impl Solution {
  pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
    g.sort();
    s.sort();
    let mut gi = 0;
    let mut si = 0;
    while gi < g.len() && si < s.len() {
      if g[gi] <= s[si] {
        gi += 1;
      }
      si += 1;
    }
    gi as i32
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(
      Solution::find_content_children(vec![1, 2, 3], vec![1, 1]),
      1
    );
    assert_eq!(
      Solution::find_content_children(vec![1, 2], vec![1, 2, 3]),
      2
    );
  }
}
