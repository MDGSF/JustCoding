// 查找第一个值等于给定值的元素
fn binarysearch_first(nums: &[i32], target: i32) -> Option<i32> {
  if nums.len() == 0 {
    return None;
  }

  let mut low = 0;
  let mut high = nums.len() as i32 - 1;
  while low <= high {
    let mid = low + ((high - low) >> 2);
    if nums[mid as usize] > target {
      high = mid - 1;
    } else if nums[mid as usize] < target {
      low = mid + 1;
    } else {
      if mid == 0 || nums[mid as usize - 1] != target {
        return Some(mid);
      } else {
        high = mid - 1;
      }
    }
  }
  None
}

// 查找最后一个值等于给定值的元素
fn binarysearch_last(nums: &[i32], target: i32) -> Option<i32> {
  let n = nums.len() as i32;
  if n == 0 {
    return None;
  }

  let mut low: i32 = 0;
  let mut high: i32 = n - 1;
  while low <= high {
    let mid = low + ((high - low) >> 2);
    if nums[mid as usize] > target {
      high = mid - 1;
    } else if nums[mid as usize] < target {
      low = mid + 1;
    } else {
      if mid == n - 1 || nums[mid as usize + 1] != target {
        return Some(mid);
      } else {
        low = mid + 1;
      }
    }
  }
  None
}

impl Solution {
  pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if let Some(left) = binarysearch_first(&nums, target) {
      let right = binarysearch_last(&nums, target).unwrap();
      return vec![left, right];
    }
    vec![-1, -1]
  }
}

struct Solution;
