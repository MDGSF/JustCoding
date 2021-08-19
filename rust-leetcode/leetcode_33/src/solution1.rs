impl Solution {
  pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    if nums.is_empty() {
      return -1;
    }

    let mut left = 0;
    let mut right = nums.len() - 1;
    while left <= right {
      let mid = left + (right - left) / 2;
      if nums[mid] == target {
        return mid as i32;
      } else if nums[left] <= nums[mid] {
        // 前半部分有序
        if target >= nums[left] && target < nums[mid] {
          // 前半部分找
          right = mid - 1;
        } else {
          // 后半部分找
          left = mid + 1;
        }
      } else {
        // 后半部分有序
        if target > nums[mid] && target <= nums[right] {
          // 后半部分找
          left = mid + 1;
        } else {
          // 前半部分找
          right = mid - 1;
        }
      }
    }

    -1
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
  }
}
