impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let n = nums.len();
        if n <= 1 {
            return;
        }

        let mut start = 0;
        let mut end = n - 1;
        let mut i = 0;
        while end > 0 && i <= end {
            if nums[i] == 0 {
                nums.swap(i, start);
                start += 1;
                i += 1;
            } else if nums[i] == 2 {
                nums.swap(i, end);
                end -= 1;
            } else {
                i += 1;
            }
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn test02() {
        let mut nums = vec![2, 0, 1];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 1, 2]);
    }

    #[test]
    fn test03() {
        let mut nums = vec![2];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![2]);
    }

    #[test]
    fn test04() {
        let mut nums = vec![2, 2];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![2, 2]);
    }
}
