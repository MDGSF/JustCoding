impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
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
        let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        assert_eq!(
            Solution::merge(intervals),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
    }

    #[test]
    fn test02() {
        let intervals = vec![vec![1, 4], vec![2, 3]];
        assert_eq!(
            Solution::merge(intervals),
            vec![vec![1, 4]]
        );
    }
}
