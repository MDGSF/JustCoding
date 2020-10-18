impl Solution {
  pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut candidates = candidates;
    candidates.sort();
    let mut result = Vec::new();
    let mut path = Vec::new();
    Self::dfs(&candidates, target, 0, &mut result, &mut path);
    result
  }

  fn dfs(
    candidates: &[i32],
    target: i32,
    begin: usize,
    result: &mut Vec<Vec<i32>>,
    path: &mut Vec<i32>,
  ) {
    if target < 0 {
      // 当前分支不符合
      return;
    }
    if target == 0 {
      // 当前分支符合，加入结果集
      result.push(path.to_vec());
      return;
    }
    for i in begin..candidates.len() {
      let new_target = target - candidates[i];
      if new_target < 0 {
        return;
      }
      path.push(candidates[i]);
      Self::dfs(candidates, new_target, i, result, path);
      path.pop();
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
