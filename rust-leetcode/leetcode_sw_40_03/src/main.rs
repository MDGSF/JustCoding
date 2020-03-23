impl Solution {
  pub fn get_least_numbers(arr: Vec<i32>, k: i32) -> Vec<i32> {
    let mut arr = arr;
    arr.sort();
    arr[..k as usize].iter().cloned().collect()
  }
}

struct Solution;

fn main() {
  let arr = vec![3, 2, 1];
  let k = 2;
  let result = Solution::get_least_numbers(arr, k);
  println!("result = {:?}", result);
}
