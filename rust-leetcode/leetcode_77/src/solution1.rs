impl Solution {
  pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut cur = Vec::new();
    Solution::recursion(n as usize, k as usize, 1, &mut cur, &mut result);
    result
  }

  fn recursion(n: usize, k: usize, first: usize, cur: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if cur.len() == k {
      result.push(cur.clone());
      return;
    }

    for i in first..=n {
      cur.push(i as i32);
      Solution::recursion(n, k, i + 1, cur, result);
      cur.pop();
    }
  }
}

pub struct Solution;
