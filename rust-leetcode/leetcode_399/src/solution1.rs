use std::collections::HashMap;

#[derive(Debug)]
pub struct Union {
  p: Vec<usize>,
  w: Vec<f64>,
}

impl Union {
  pub fn new(cap: usize) -> Union {
    let p = (0..cap).collect::<Vec<usize>>();
    Union {
      p,
      w: vec![1f64; cap],
    }
  }

  pub fn root(&mut self, i: usize) -> (usize, f64) {
    let mut cur = i;
    let mut val = 1f64;
    while self.p[cur] != cur {
      val *= self.w[cur];
      cur = self.p[cur];
    }

    self.p[i] = cur;
    self.w[i] = val;

    (cur, val)
  }

  pub fn add(&mut self, child: usize, parent: usize, val: f64) {
    let (child_root, child_weight) = self.root(child);
    let (parent_root, parent_weight) = self.root(parent);
    if child_root == parent_root {
      // 在同一个并查集里面，不用修改
    } else {
      self.p[child] = parent_root;
      self.w[child] = val * parent_weight;

      if child != child_root {
        self.p[child_root] = self.p[child];
        self.w[child_root] = 1f64 / child_weight * self.w[child];
      }
    }
  }

  pub fn query(&mut self, n1: usize, n2: usize) -> f64 {
    if n1 == n2 {
      return 1f64;
    }
    let (n1_root, n1_weight) = self.root(n1);
    let (n2_root, n2_weight) = self.root(n2);
    if n1_root == n2_root {
      n1_weight / n2_weight
    } else {
      -1_f64
    }
  }
}

impl Solution {
  pub fn calc_equation(
    equations: Vec<Vec<String>>,
    values: Vec<f64>,
    queries: Vec<Vec<String>>,
  ) -> Vec<f64> {
    let mut id: HashMap<String, usize> = HashMap::new();
    for i in 0..equations.len() {
      let a = equations[i][0].clone();
      let b = equations[i][1].clone();
      if !id.contains_key(&a) {
        id.insert(a, id.len());
      }
      if !id.contains_key(&b) {
        id.insert(b, id.len());
      }
    }

    let mut u = Union::new(id.len());

    for i in 0..equations.len() {
      let a = *id.get(&equations[i][0]).unwrap();
      let b = *id.get(&equations[i][1]).unwrap();
      let val = values[i];
      u.add(a, b, val);
    }

    let mut result = Vec::new();
    for i in 0..queries.len() {
      let a = queries[i][0].clone();
      let b = queries[i][1].clone();

      if !id.contains_key(&a) || !id.contains_key(&b) {
        result.push(-1f64);
        continue;
      }

      let aid = *id.get(&a).unwrap();
      let bid = *id.get(&b).unwrap();
      let val = u.query(aid, bid);
      result.push(val);
    }
    result
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

  #[test]
  fn test_2() {
    let equations = vec![
      vec!["a".to_string(), "b".to_string()],
      vec!["b".to_string(), "c".to_string()],
      vec!["bc".to_string(), "cd".to_string()],
    ];
    let values = vec![1.5, 2.5, 5.0];
    let queries = vec![
      vec!["a".to_string(), "c".to_string()],
      vec!["c".to_string(), "b".to_string()],
      vec!["bc".to_string(), "cd".to_string()],
      vec!["cd".to_string(), "bc".to_string()],
    ];
    let result = vec![3.75000, 0.40000, 5.00000, 0.20000];
    assert_eq!(Solution::calc_equation(equations, values, queries), result);
  }

  #[test]
  fn test_3() {
    let equations = vec![vec!["a".to_string(), "b".to_string()]];
    let values = vec![0.5];
    let queries = vec![
      vec!["a".to_string(), "b".to_string()],
      vec!["b".to_string(), "a".to_string()],
      vec!["a".to_string(), "c".to_string()],
      vec!["x".to_string(), "y".to_string()],
    ];
    let result = vec![0.50000, 2.00000, -1.00000, -1.00000];
    assert_eq!(Solution::calc_equation(equations, values, queries), result);
  }

  #[test]
  fn test_4() {
    let equations = vec![
      vec!["x1".to_string(), "x2".to_string()],
      vec!["x2".to_string(), "x3".to_string()],
      vec!["x1".to_string(), "x4".to_string()],
      vec!["x2".to_string(), "x5".to_string()],
    ];
    let values = vec![3.0, 0.5, 3.4, 5.6];
    let queries = vec![
      vec!["x2".to_string(), "x4".to_string()],
      vec!["x1".to_string(), "x5".to_string()],
      vec!["x1".to_string(), "x3".to_string()],
      vec!["x5".to_string(), "x5".to_string()],
      vec!["x5".to_string(), "x1".to_string()],
      vec!["x3".to_string(), "x4".to_string()],
      vec!["x4".to_string(), "x3".to_string()],
      vec!["x6".to_string(), "x6".to_string()],
      vec!["x0".to_string(), "x0".to_string()],
    ];
    let result = vec![
      1.13333, 16.8, 1.5, 1.0, 0.05952, 2.26667, 0.44118, -1.0, -1.0,
    ];
    assert_eq!(Solution::calc_equation(equations, values, queries), result);
  }
}
