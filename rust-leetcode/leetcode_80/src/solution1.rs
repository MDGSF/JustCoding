impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let n = nums.len();
        let mut pre_num = nums[0];
        let mut count = 1;
        let mut idx = 1;
        for i in 1..n {
            nums[idx] = nums[i];
            if nums[i] == pre_num {
                count += 1;
                if count <= 2 {
                    idx += 1;
                }
            } else {
                pre_num = nums[i];
                idx += 1;
                count = 1;
            }
        }
        idx as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        let n = Solution::remove_duplicates(&mut nums) as usize;
        assert_eq!(n, 5);
        assert_eq!(&nums[0..n], &[1, 1, 2, 2, 3]);
    }

    #[test]
    fn test02() {
        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        let n = Solution::remove_duplicates(&mut nums) as usize;
        assert_eq!(n, 7);
        assert_eq!(&nums[0..n], &[0, 0, 1, 1, 2, 3, 3]);
    }
}
