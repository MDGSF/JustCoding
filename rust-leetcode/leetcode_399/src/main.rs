use leetcode_399::solution1::Solution;

fn main() {
  let equations = vec![
    vec!["a".to_string(), "b".to_string()],
    vec!["b".to_string(), "c".to_string()],
  ];
  let values = vec![2.0, 3.0];
  let queries = vec![
    vec!["a".to_string(), "c".to_string()],
    vec!["b".to_string(), "a".to_string()],
    vec!["a".to_string(), "e".to_string()],
    vec!["a".to_string(), "a".to_string()],
    vec!["x".to_string(), "x".to_string()],
  ];
  let result = Solution::calc_equation(equations, values, queries);

  println!("result = {:?}", result);
}
