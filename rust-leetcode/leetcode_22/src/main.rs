impl Solution {
  pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    Solution::dfs(n, String::from(""), 0, 0, &mut result);
    result
  }

  pub fn dfs(n: i32, node: String, left: i32, right: i32, mut result: &mut Vec<String>) {
    if left == n && right == n {
      result.push(node);
      return;
    }
    if left < n {
      Solution::dfs(n, node.clone() + "(", left + 1, right, &mut result);
    }
    if right < left {
      Solution::dfs(n, node.clone() + ")", left, right + 1, &mut result);
    }
  }
}

struct Solution;

fn main() {
  let result = Solution::generate_parenthesis(3);
  println!("result = {:?}", result);
}
