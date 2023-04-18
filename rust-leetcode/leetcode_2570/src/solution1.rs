impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut map = vec![0; 1024];
        for entry in nums1 {
            map[entry[0] as usize] += entry[1];
        }
        for entry in nums2 {
            map[entry[0] as usize] += entry[1];
        }

        let mut result = vec![];
        for (i, &val) in map.iter().enumerate() {
            if val > 0 {
                result.push(vec![i as i32, val]);
            }
        }
        result
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let nums1 = vec![vec![1, 2], vec![2, 3], vec![4, 5]];
        let nums2 = vec![vec![1, 4], vec![3, 2], vec![4, 1]];
        let result = vec![vec![1, 6], vec![2, 3], vec![3, 2], vec![4, 6]];
        assert_eq!(Solution::merge_arrays(nums1, nums2), result);
    }
}
