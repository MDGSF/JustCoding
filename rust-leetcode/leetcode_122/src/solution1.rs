impl Solution {
  pub fn max_profit(prices: Vec<i32>) -> i32 {
    prices
      .windows(2)
      .filter(|w| w[1] > w[0])
      .map(|w| w[1] - w[0])
      .sum()
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
  }
}
