use std::collections::HashMap;

impl Solution {
  pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut m = HashMap::new();
    Self::recursion(&coins, amount, &mut m)
  }

  fn recursion(coins: &[i32], amount: i32, m: &mut HashMap<i32, i32>) -> i32 {
    if amount < 0 {
      return -1;
    }
    if amount == 0 {
      return 0;
    }

    if let Some(&count) = m.get(&amount) {
      return count;
    }

    let mut result = std::i32::MAX;
    for &coin in coins.iter() {
      let sub = Self::recursion(coins, amount - coin, m);
      if sub == -1 {
        continue;
      }
      result = result.min(sub + 1);
    }
    let count = if result == std::i32::MAX { -1 } else { result };
    m.insert(amount, count);
    return count;
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
    assert_eq!(Solution::coin_change(vec![2], 3), -1);
  }
}
