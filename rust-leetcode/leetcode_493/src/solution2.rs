impl Solution {
  pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
      return 0;
    }
    let mut nums = nums.iter().map(|&num| num as isize).collect::<Vec<isize>>();
    Solution::reverse_pairs_inner(&mut nums[..]) as i32
  }

  fn reverse_pairs_inner(nums: &mut [isize]) -> usize {
    if nums.len() <= 1 {
      return 0;
    }
    let length = nums.len() / 2;
    let left = Solution::reverse_pairs_inner(&mut nums[..length]);
    let right = Solution::reverse_pairs_inner(&mut nums[length..]);
    let count = Solution::count(&nums[..length], &nums[length..]);
    nums.sort();
    left + right + count
  }

  fn count(first: &[isize], second: &[isize]) -> usize {
    let mut i = (first.len() - 1) as i32;
    let mut j = (second.len() - 1) as i32;
    let mut result = 0;
    while i >= 0_i32 && j >= 0_i32 {
      if first[i as usize] > second[j as usize] * 2 {
        result += j + 1;
        i -= 1;
      } else {
        j -= 1;
      }
    }
    result as usize
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
