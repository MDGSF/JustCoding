use std::collections::BinaryHeap;

impl Solution {
  pub fn get_least_numbers(arr: Vec<i32>, k: i32) -> Vec<i32> {
    let mut heap = BinaryHeap::with_capacity(k as usize);
    for num in arr {
      if heap.len() < k as usize {
        heap.push(num);
      } else {
        if !heap.is_empty() && *heap.peek().unwrap() > num {
          heap.pop();
          heap.push(num);
        }
      }
    }
    heap.iter().cloned().collect()
  }
}

struct Solution;

fn main() {
  let arr = vec![3, 2, 1];
  let k = 2;
  let result = Solution::get_least_numbers(arr, k);
  println!("result = {:?}", result);
}
