impl Solution {
  pub fn find_best_value(arr: Vec<i32>, target: i32) -> i32 {
    let mut arr = arr;
    arr.sort();
    let mut prefix = vec![0; arr.len() + 1];
    for (i, &num) in arr.iter().enumerate() {
      prefix[i + 1] = prefix[i] + num
    }

    let mut left = 0;
    let mut right = arr[arr.len() - 1];
    let mut result = -1;
    while left <= right {
      let val = (left + right) / 2;
      let i = Solution::binarysearch_first_ge(&arr, val);
      let cur = prefix[i] + (arr.len() - i) as i32 * val;
      if cur <= target {
        result = val;
        left = val + 1;
      } else {
        right = val - 1;
      }
    }

    let small = Solution::check(&arr, result);
    let big = Solution::check(&arr, result + 1);
    if (small - target).abs() <= (big - target).abs() {
      return result;
    }
    return result + 1;
  }

  pub fn check(arr: &[i32], x: i32) -> i32 {
    let mut result = 0;
    for &num in arr.iter() {
      if num >= x {
        result += x;
      } else {
        result += num;
      }
    }
    result
  }

  // 返回第一个可以插入 v 的下标位置
  pub fn binarysearch_first_ge(a: &[i32], v: i32) -> usize {
    if a.len() == 0 {
      return 0;
    }
    let mut low = 0;
    let mut high = a.len() - 1;
    while low <= high {
      let mid = low + (high - low) / 2;
      if a[mid] >= v {
        if mid == 0 || (a[mid - 1] < v) {
          return mid;
        } else {
          high = mid - 1;
        }
      } else {
        low = mid + 1;
      }
    }
    return a.len();
  }
}

struct Solution;

fn main() {
  // let arr = vec![4, 9, 3];
  // let target = 10;

  // let arr = vec![2, 3, 5];
  // let target = 10;

  let arr = vec![60864, 25176, 27249, 21296, 20204];
  let target = 56803;

  let result = Solution::find_best_value(arr, target);
  println!("Hello, world!, result = {}", result);
}
