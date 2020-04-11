impl Solution {
  pub fn super_egg_drop(k: i32, n: i32) -> i32 {
    let k: usize = k as usize;
    let mut m: usize = 0;
    let mut dp = vec![vec![0; (n + 1) as usize]; k + 1];
    while dp[k as usize][m] < n {
      m += 1;
      for row in 1..k + 1 {
        dp[row][m] = dp[row][m - 1] + dp[row - 1][m - 1] + 1;
      }
    }
    m as i32
  }
}

struct Solution;

fn main() {
  let result = Solution::super_egg_drop(3, 14);
  println!("result = {}", result);
}
