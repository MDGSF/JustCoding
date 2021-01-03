impl Solution {
  pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
    if s.is_empty() {
      return vec![0, 0];
    }
    let mut lines = 1;
    let mut cur_line_num = 0;
    s.as_bytes().iter().for_each(|&c| {
      let cur_num = widths[(c - b'a') as usize];
      if cur_line_num + cur_num > 100 {
        lines += 1;
        cur_line_num = cur_num;
      } else {
        cur_line_num += cur_num;
      }
    });
    vec![lines, cur_line_num]
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    assert_eq!(
      Solution::number_of_lines(
        vec![
          10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
          10, 10, 10, 10
        ],
        "abcdefghijklmnopqrstuvwxyz".to_string()
      ),
      vec![3, 60]
    );

    assert_eq!(
      Solution::number_of_lines(
        vec![
          4, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
          10, 10, 10, 10,
        ],
        "bbbcccdddaaa".to_string()
      ),
      vec![2, 4]
    );
  }
}
