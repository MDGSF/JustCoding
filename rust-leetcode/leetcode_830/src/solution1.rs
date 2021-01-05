impl Solution {
  pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
    let s = s.as_bytes();
    (1..=s.len())
      .fold((Vec::new(), 0), |(mut res, pre), cur| {
        if cur == s.len() {
          if s[pre] == s[cur - 1] && cur - pre >= 3 {
            res.push(vec![pre as i32, (cur - 1) as i32]);
            return (res, cur);
          } else {
            return (res, pre);
          }
        }

        if s[pre] != s[cur] {
          if cur - pre >= 3 {
            res.push(vec![pre as i32, (cur - 1) as i32]);
          }
          (res, cur)
        } else {
          (res, pre)
        }
      })
      .0
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    assert_eq!(
      Solution::large_group_positions("abbxxxxzzy".to_string()),
      vec![vec![3, 6]]
    );

    assert_eq!(
      Solution::large_group_positions("abc".to_string()),
      Vec::<Vec<i32>>::new()
    );

    assert_eq!(
      Solution::large_group_positions("abcdddeeeeaabbbcd".to_string()),
      vec![vec![3, 5], vec![6, 9], vec![12, 14]]
    );

    assert_eq!(
      Solution::large_group_positions("aba".to_string()),
      Vec::<Vec<i32>>::new()
    );

    assert_eq!(
      Solution::large_group_positions("aaa".to_string()),
      vec![vec![0, 2]]
    );
  }
}
