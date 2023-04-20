use std::collections::BTreeSet;

impl Solution {
    pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
        nums.into_iter()
            .map(|v| v.into_iter().collect::<BTreeSet<_>>())
            .reduce(|acc, s| &acc & &s)
            .unwrap()
            .into_iter()
            .collect()
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
