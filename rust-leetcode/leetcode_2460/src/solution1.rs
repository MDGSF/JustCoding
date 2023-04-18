impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        for i in 0..(n - 1) {
            if nums[i] == nums[i + 1] {
                nums[i] *= 2;
                nums[i + 1] = 0;
            }
        }

        let mut j = 0;
        for i in 0..n {
            if nums[i] != 0 {
                nums.swap(i, j);
                j += 1;
            }
        }

        nums
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(
            Solution::apply_operations(vec![1, 2, 2, 1, 1, 0]),
            vec![1, 4, 2, 0, 0, 0]
        );
    }

    #[test]
    fn test02() {
        assert_eq!(Solution::apply_operations(vec![0, 1]), vec![1, 0]);
    }
}
