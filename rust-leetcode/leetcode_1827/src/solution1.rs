impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let len = nums.len();
        for i in 1..len {
            if nums[i - 1] >= nums[i] {
                count += nums[i - 1] + 1 - nums[i];
                nums[i] = nums[i - 1] + 1;
            }
        }
        count
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_operations(vec![1, 1, 1]), 3);
        assert_eq!(Solution::min_operations(vec![1, 5, 2, 4, 1]), 14);
        assert_eq!(Solution::min_operations(vec![8]), 0);
    }
}
