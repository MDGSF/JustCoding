impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold((nums.iter().min().unwrap(), 0), |(min, acc), num| {
                (min, acc + (num - min))
            })
            .1
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
