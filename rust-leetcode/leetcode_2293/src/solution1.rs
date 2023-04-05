impl Solution {
    pub fn min_max_game(mut nums: Vec<i32>) -> i32 {
        let mut n = nums.len();
        while n > 1 {
            let mut flag = true;
            let mut i = 0;
            while i < n {
                if flag {
                    nums[i / 2] = nums[i].min(nums[i + 1]);
                    flag = false;
                } else {
                    nums[i / 2] = nums[i].max(nums[i + 1]);
                    flag = true;
                }
                i += 2;
            }
            n = n / 2;
        }
        nums[0]
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(Solution::min_max_game(vec![1, 3, 5, 2, 4, 8, 2, 2]), 1);
        assert_eq!(Solution::min_max_game(vec![3]), 3);
    }
}
