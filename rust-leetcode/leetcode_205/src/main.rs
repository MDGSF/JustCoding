use leetcode_205::solution1::Solution;

fn main() {
  let s = "egg".to_string();
  let t = "add".to_string();
  let result = Solution::is_isomorphic(s, t);
  println!("result = {}", result);
}
