impl Solution {
    pub fn unequal_triplets(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut count = 0;
        for i in 0..n {
            for j in (i + 1)..n {
                for k in (j + 1)..n {
                    if nums[i] != nums[j] && nums[j] != nums[k] && nums[i] != nums[k] {
                        count += 1;
                    }
                }
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
    fn test01() {
        assert_eq!(Solution::unequal_triplets(vec![4, 4, 2, 4, 3]), 3);
    }

    #[test]
    fn test02() {
        assert_eq!(Solution::unequal_triplets(vec![1, 1, 1, 1, 1]), 0);
    }
}
