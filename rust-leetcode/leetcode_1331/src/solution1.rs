use std::collections::HashMap;

impl Solution {
  pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
    let mut arr_sorted = arr.clone();
    arr_sorted.sort();

    let mut m = HashMap::new();
    let mut idx = 0;
    for i in 0..arr_sorted.len() {
      m.entry(arr_sorted[i]).or_insert_with(|| {
        idx += 1;
        idx
      });
    }

    let mut result: Vec<i32> = Vec::with_capacity(arr.len());
    for num in arr {
      result.push(*m.get(&num).unwrap());
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
    assert_eq!(
      Solution::array_rank_transform(vec![40, 10, 20, 30]),
      vec![4, 1, 2, 3]
    );

    assert_eq!(
      Solution::array_rank_transform(vec![100, 100, 100]),
      vec![1, 1, 1]
    );

    assert_eq!(
      Solution::array_rank_transform(vec![37, 12, 28, 9, 100, 56, 80, 5, 12]),
      vec![5, 3, 4, 2, 8, 6, 7, 1, 3]
    );
  }
}
