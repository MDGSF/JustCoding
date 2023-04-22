impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let n = nums.len() as i32;
        if n == 0 {
            return false;
        }
        if n == 1 {
            return nums[0] == target;
        }

        let mut left: i32 = 0;
        let mut right: i32 = n - 1;
        while left <= right {
            let mid = (left + right) / 2;
            println!("left:{}, right:{}, mid:{}", left, right, mid);
            if nums[mid as usize] == target {
                return true;
            }

            if nums[left as usize] == nums[mid as usize]
                && nums[mid as usize] == nums[right as usize]
            {
                left += 1;
                right -= 1;
            } else if nums[left as usize] <= nums[mid as usize] {
                // 分割点在右侧
                if target >= nums[left as usize] && target < nums[mid as usize] {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            } else {
                // 分割点在左侧
                if target > nums[mid as usize] && target <= nums[right as usize] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
        }
        false
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0), true);
    }

    #[test]
    fn test02() {
        assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3), false);
    }

    #[test]
    fn test03() {
        assert_eq!(Solution::search(vec![], 3), false);
        assert_eq!(Solution::search(vec![1], 3), false);
    }

    #[test]
    fn test04() {
        assert_eq!(Solution::search(vec![1, 0, 1, 1, 1], 0), true);
    }

    #[test]
    fn test05() {
        assert_eq!(Solution::search(vec![5, 1, 3], 3), true);
    }
}
