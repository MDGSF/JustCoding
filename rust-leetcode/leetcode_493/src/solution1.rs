impl Solution {
  pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
      return 0;
    }
    let mut nums = nums.iter().map(|&num| num as isize).collect::<Vec<isize>>();
    let start = 0;
    let end = nums.len() - 1;
    Solution::reverse_pairs_inner(&mut nums, start, end) as i32
  }

  fn reverse_pairs_inner(nums: &mut [isize], start: usize, end: usize) -> usize {
    if start >= end {
      return 0;
    }
    let length = (end - start) / 2;
    let left = Solution::reverse_pairs_inner(nums, start, start + length);
    let right = Solution::reverse_pairs_inner(nums, start + length + 1, end);
    let mut i = (start + length) as i32;
    let mut j = end as i32;
    let mut count = 0;
    while i >= start as i32 && j >= (start + length + 1) as i32 {
      if nums[i as usize] > nums[j as usize] * 2 {
        count += j as usize - start - length;
        i -= 1;
      } else {
        j -= 1;
      }
    }
    nums[start..(end + 1)].sort();
    left + right + count
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    assert_eq!(Solution::reverse_pairs(vec![]), 0);

    assert_eq!(Solution::reverse_pairs(vec![1, 3, 2, 3, 1]), 2);

    assert_eq!(Solution::reverse_pairs(vec![2, 4, 3, 5, 1]), 3);

    assert_eq!(
      Solution::reverse_pairs(vec![
        2147483647, 2147483647, 2147483647, 2147483647, 2147483647, 2147483647
      ]),
      0
    );

    assert_eq!(
      Solution::reverse_pairs(vec![
        2147483647,
        2147483647,
        -2147483647,
        -2147483647,
        -2147483647,
        2147483647
      ]),
      9
    );

    assert_eq!(Solution::reverse_pairs(vec![5, 4, 3, 2, 1]), 4);
  }
}
