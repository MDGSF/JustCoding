impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        intervals.push(new_interval);
        intervals.sort_unstable_by(|a, b| a.cmp(b));
        intervals.iter().skip(1).fold(
            vec![intervals[0].iter().cloned().collect()],
            |mut acc, interval| {
                let n = acc.len();
                if acc[n - 1][1] < interval[0] {
                    acc.push(interval.to_vec());
                } else {
                    acc[n - 1][1] = acc[n - 1][1].max(interval[1]);
                }
                acc
            },
        )
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
