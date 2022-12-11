impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        if nums.len() <= 0 {
            return 0;
        }
        let (total, _) = nums
            .iter()
            .fold((0, nums[0] - 1), |(mut total, mut pre), &num| {
                if pre >= num {
                    total += pre - num + 1;
                    pre = pre + 1;
                } else {
                    pre = num;
                }
                (total, pre)
            });
        total
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
