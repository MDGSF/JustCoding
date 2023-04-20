use std::collections::BTreeSet;

impl Solution {
    pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
        nums.iter()
            .skip(1)
            .fold(
                nums[0].iter().cloned().collect::<BTreeSet<i32>>(),
                |set, arr| {
                    let s = arr.iter().cloned().collect::<BTreeSet<i32>>();
                    set.intersection(&s).cloned().collect()
                },
            )
            .iter()
            .cloned()
            .collect::<Vec<i32>>()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let nums = vec![vec![3, 1, 2, 4, 5], vec![1, 2, 3, 4], vec![3, 4, 5, 6]];
        assert_eq!(Solution::intersection(nums), vec![3, 4]);
    }
}
