impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut idx = 0;
        for i in 0..nums.len() {
            if idx < 2 || nums[i] != nums[idx - 2] {
                nums[idx] = nums[i];
                idx += 1;
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
