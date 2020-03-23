impl Solution {
  pub fn get_least_numbers(arr: Vec<i32>, k: i32) -> Vec<i32> {
    let mut arr = arr;
    let k = k - 1;
    let mut start: i32 = 0;
    let mut end: i32 = (arr.len() - 1) as i32;
    let mut partition_index = -1;
    while partition_index != k {
      partition_index = Solution::partition(&mut arr, start as usize, end as usize);
      if partition_index < k {
        start = partition_index + 1;
      } else {
        end = partition_index - 1;
      }
    }
    arr[..((k + 1) as usize)].iter().cloned().collect()
  }

  fn partition(arr: &mut [i32], start: usize, end: usize) -> i32 {
    let pivot = arr[end];
    let mut partition_index = start;
    for i in start..end {
      if arr[i] < pivot {
        arr.swap(i, partition_index);
        partition_index += 1;
      }
    }
    arr.swap(partition_index, end);
    partition_index as i32
  }
}

struct Solution;

fn main() {
  let arr = vec![3, 2, 1, 11, 10, 9];
  let k = 2;
  let result = Solution::get_least_numbers(arr, k);
  println!("result = {:?}", result);
}
