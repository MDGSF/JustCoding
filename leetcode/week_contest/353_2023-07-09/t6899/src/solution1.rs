use std::collections::VecDeque;

impl Solution {
    pub fn maximum_jumps(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();

        let mut dp = vec![-1; n];
        dp[0] = 0;

        let mut deque: VecDeque<usize> = VecDeque::new();
        deque.push_back(0);

        while !deque.is_empty() {
            let i = deque.pop_front().unwrap();
            for j in (i + 1)..n {
                if (nums[j] - nums[i]).abs() <= target {
                    if dp[i] + 1 > dp[j] {
                        dp[j] = dp[i] + 1;
                        deque.push_back(j);
                    }
                }
            }
        }

        dp[n - 1]
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(Solution::maximum_jumps(vec![1, 3, 6, 4, 1, 2], 2), 3);
    }

    #[test]
    fn test02() {
        assert_eq!(Solution::maximum_jumps(vec![1, 3, 6, 4, 1, 2], 3), 5);
    }

    #[test]
    fn test03() {
        assert_eq!(Solution::maximum_jumps(vec![1, 3, 6, 4, 1, 2], 0), -1);
    }
}
