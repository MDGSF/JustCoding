impl Solution {
  pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
    let n = s1.len();
    let m = s2.len();
    let t = s3.len();

    if n + m != t {
      return false;
    }

    let p1 = s1.as_bytes();
    let p2 = s2.as_bytes();
    let p3 = s3.as_bytes();

    let mut dp = vec![false; s2.len() + 1];
    dp[0] = true;

    for i in 0..=n {
      for j in 0..=m {
        let p = if i + j >= 1 {
          i + j - 1
        } else {
          continue;
        };
        if i > 0 {
          dp[j] = dp[j] && p1[i - 1] == p3[p]
        }
        if j > 0 {
          dp[j] = dp[j] || (dp[j - 1] && p2[j - 1] == p3[p])
        }
      }
    }

    dp[m]
  }
}

struct Solution;

fn main() {
  //let s1 = "aabcc".to_string();
  //let s2 = "dbbca".to_string();
  //let s3 = "aadbbcbcac".to_string();

  //let s1 = "aabcc".to_string();
  //let s2 = "dbbca".to_string();
  //let s3 = "aadbbbaccc".to_string();

  let s1 = "".to_string();
  let s2 = "b".to_string();
  let s3 = "b".to_string();

  let result = Solution::is_interleave(s1, s2, s3);
  println!("result = {}", result);
}
