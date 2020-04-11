impl Solution {
  pub fn min_distance(word1: String, word2: String) -> i32 {
    use std::cmp::min;
    let chars1: Vec<char> = word1.chars().collect();
    let chars2: Vec<char> = word2.chars().collect();
    let rows = word1.len() + 1;
    let cols = word2.len() + 1;
    let mut dp = vec![vec![0; cols]; rows];
    for row in 1..rows {
      dp[row][0] = row;
    }
    for col in 1..cols {
      dp[0][col] = col;
    }
    for row in 1..rows {
      for col in 1..cols {
        if chars1[row - 1] == chars2[col - 1] {
          dp[row][col] = dp[row - 1][col - 1];
        } else {
          dp[row][col] = 1
            + min(
              min(dp[row - 1][col - 1], dp[row - 1][col]),
              dp[row][col - 1],
            );
        }
      }
    }
    dp[rows - 1][cols - 1] as i32
  }
}

struct Solution;

fn main() {
  let result = Solution::min_distance("horse".to_string(), "ros".to_string());
  println!("result = {}", result);
}
