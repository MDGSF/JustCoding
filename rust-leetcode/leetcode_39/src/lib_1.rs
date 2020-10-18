impl Solution {
  pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut result = vec![vec![0; 0]; 0];
    let mut combine = Vec::new();
    Self::dfs(&candidates, target, &mut result, &mut combine, 0);
    result
  }

  fn dfs(
    candidates: &[i32],
    target: i32,
    mut result: &mut Vec<Vec<i32>>,
    mut combine: &mut Vec<i32>,
    idx: usize,
  ) {
    if idx == candidates.len() {
      return;
    }

    if target == 0 {
      result.push(combine.to_vec());
      return;
    }

    // 不选第 idx 个元素
    Self::dfs(candidates, target, &mut result, &mut combine, idx + 1);

    // 选第 idx 个元素
    if target - candidates[idx] >= 0 {
      combine.push(candidates[idx]);
      Self::dfs(
        candidates,
        target - candidates[idx],
        &mut result,
        &mut combine,
        idx,
      );
      combine.pop();
    }
  }
}

struct Solution;

fn main() {
  let candidates = vec![2, 3, 5];
  let target = 8;
  let result = Solution::combination_sum(candidates, target);
  println!("result = {:?}", result);
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test1() {
    let candidates = vec![2, 3, 6, 7];
    let target = 7;
    let result = Solution::combination_sum(candidates, target);
    assert_eq!(result, vec![vec![7], vec![2, 2, 3]]);
  }
}
