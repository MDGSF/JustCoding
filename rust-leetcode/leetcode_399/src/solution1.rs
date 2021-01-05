use std::collections::HashMap;

impl Solution {
  pub fn calc_equation(
    equations: Vec<Vec<String>>,
    values: Vec<f64>,
    queries: Vec<Vec<String>>,
  ) -> Vec<f64> {
    let mut m: HashMap<String, f64> = HashMap::new();
    let values_len = values.len();
    for i in 0..values_len {
      let equation = equations[i];
      let value = values[i];

      m.insert(equation[0] + equation[1], value);
      m.insert(equation[1] + equation[0], 1f64 / value);
    }
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
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
    let result = vec![6.00000, 0.50000, -1.00000, 1.00000, -1.00000];
    assert_eq!(Solution::calc_equation(equations, values, queries), result);
  }
}
