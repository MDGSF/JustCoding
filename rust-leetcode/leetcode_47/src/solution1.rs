use std::collections::HashMap;

impl Solution {
  pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut m: HashMap<i32, i32> = nums.iter().fold(HashMap::new(), |mut acc, n| {
      *acc.entry(*n).or_insert(0) += 1;
      acc
    });
    let m_keys: Vec<i32> = m.keys().map(|&e| e).collect();

    let mut cur = Vec::new();
    let mut result = Vec::new();
    Solution::recursion(nums.len(), &m_keys, &mut m, &mut cur, &mut result);
    result
  }

  fn recursion(
    nums_len: usize,
    m_keys: &Vec<i32>,
    m: &mut HashMap<i32, i32>,
    cur: &mut Vec<i32>,
    result: &mut Vec<Vec<i32>>,
  ) {
    if nums_len == cur.len() {
      result.push(cur.clone());
      return;
    }

    for key in m_keys.iter() {
      if *m.get_mut(key).unwrap() > 0 {
        cur.push(*key);
        *m.get_mut(key).unwrap() -= 1;

        Solution::recursion(nums_len, m_keys, m, cur, result);

        cur.pop();
        *m.get_mut(key).unwrap() += 1;
      }
    }
  }
}

pub struct Solution;
