impl Solution {
  pub fn modify_string(mut s: String) -> String {
    let bytes = unsafe { s.as_bytes_mut() };
    for i in 0..bytes.len() {
      if bytes[i] == b'?' {
        let left = if i == 0 { None } else { Some(bytes[i - 1]) };
        let right = if i == bytes.len() - 1 {
          None
        } else {
          Some(bytes[i + 1])
        };
        if let Some(x) = (b'a'..=b'z').find(|&x| Some(x) != left && Some(x) != right) {
          bytes[i] = x;
        }
      }
    }
    s
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    assert_eq!(Solution::modify_string("?zs".to_string()), "azs");
    assert_eq!(Solution::modify_string("ubv?w".to_string()), "ubvaw");
    assert_eq!(Solution::modify_string("j?qg??b".to_string()), "jaqgacb");
    assert_eq!(
      Solution::modify_string("??yw?ipkj?".to_string()),
      "abywaipkja"
    );
  }
}
