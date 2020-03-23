use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
  pub fn get_least_numbers(arr: Vec<i32>, k: i32) -> Vec<i32> {
    let mut heap = BinaryHeap::with_capacity(arr.len());
    for num in arr {
      heap.push(Reverse(num));
    }
    let mut result = Vec::with_capacity(k as usize);
    for _ in 0..k {
      result.push(heap.pop().unwrap().0);
    }
    result
  }
}

struct Solution;

fn main() {
  let arr = vec![3, 2, 1];
  let k = 2;
  let result = Solution::get_least_numbers(arr, k);
  println!("result = {:?}", result);
}
