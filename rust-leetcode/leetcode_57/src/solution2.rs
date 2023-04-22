impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut left = new_interval[0];
        let mut right = new_interval[1];
        let mut placed = false;
        let mut result = Vec::new();
        for interval in intervals.iter() {
            if interval[0] > right {
                // 无交集
                if !placed {
                    result.push(vec![left, right]);
                    placed = true;
                }
                result.push(interval.to_vec());
            } else if interval[1] < left {
                // 无交集
                result.push(interval.to_vec());
            } else {
                // 有交集
                left = left.min(interval[0]);
                right = right.max(interval[1]);
            }
        }
        if !placed {
            result.push(vec![left, right]);
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
        let intervals = vec![vec![1, 3], vec![6, 9]];
        let new_interval = vec![2, 5];
        assert_eq!(
            Solution::insert(intervals, new_interval),
            vec![vec![1, 5], vec![6, 9]]
        );
    }
}
