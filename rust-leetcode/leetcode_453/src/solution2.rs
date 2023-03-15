impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let min: i32 = *nums.iter().min().unwrap();
        let sum: i32 = nums.iter().sum();
        let n: i32 = nums.len() as i32;
        sum - min * n
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let nums = vec![1, 2, 3];
        let result = Solution::min_moves(nums);
        assert_eq!(result, 3);
    }
}
