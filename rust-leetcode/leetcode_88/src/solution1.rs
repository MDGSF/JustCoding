impl Solution {
  pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut idx = m + n - 1;
    let mut idx1 = m - 1;
    let mut idx2 = n - 1;
    while idx1 >= 0 && idx2 >= 0 {
      if nums1[idx1 as usize] > nums2[idx2 as usize] {
        nums1[idx as usize] = nums1[idx1 as usize];
        idx -= 1;
        idx1 -= 1;
      } else {
        nums1[idx as usize] = nums2[idx2 as usize];
        idx -= 1;
        idx2 -= 1;
      }
    }
    while idx1 >= 0 {
      nums1[idx as usize] = nums1[idx1 as usize];
      idx -= 1;
      idx1 -= 1;
    }
    while idx2 >= 0 {
      nums1[idx as usize] = nums2[idx2 as usize];
      idx -= 1;
      idx2 -= 1;
    }
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];
    Solution::merge(&mut nums1, 3, &mut nums2, 3);
    assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
  }
}
