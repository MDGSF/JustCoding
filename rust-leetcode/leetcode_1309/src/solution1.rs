impl Solution {
  pub fn freq_alphabets(s: String) -> String {
    let s = s.as_bytes();
    let mut result: Vec<u8> = Vec::new();
    for &c in s {
      if c != b'#' {
        result.push(c);
      } else {
        let num: u8 = String::from_utf8(result[result.len() - 2..].to_vec())
          .unwrap()
          .parse()
          .unwrap();
        let cnum: u8 = b'a' + num - 1;
        result.pop();
        result.pop();
        result.push(cnum);
      }
    }
    for c in result.iter_mut() {
      if *c >= b'1' && *c <= b'9' {
        *c = b'a' + *c - b'1';
      }
    }
    unsafe { String::from_utf8_unchecked(result) }
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    assert_eq!(
      Solution::freq_alphabets("10#11#12".to_string()),
      "jkab".to_string()
    );

    assert_eq!(
      Solution::freq_alphabets("1326#".to_string()),
      "acz".to_string()
    );

    assert_eq!(Solution::freq_alphabets("25#".to_string()), "y".to_string());

    assert_eq!(
      Solution::freq_alphabets(
        "12345678910#11#12#13#14#15#16#17#18#19#20#21#22#23#24#25#26#".to_string()
      ),
      "abcdefghijklmnopqrstuvwxyz".to_string()
    );
  }
}
