/*
d[i]: 以 i 结尾分割的最大和
d[i] = max(d[j] + maxValue * (i - j))
maxValue = max(arr[j+1], ..., arr[i])
j∈[i-k,i-1]
*/
impl Solution {
    pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
        let n = arr.len();
        let mut d = vec![0; n + 1];
        for i in 1..=n {
            let mut max_value = arr[i - 1];
            let k = k as usize;
            let start_idx = if i > k { i - k } else { 0 };
            for j in (start_idx..=i - 1).rev() {
                d[i] = d[i].max(d[j] + max_value * (i - j) as i32);
                if j > 0 {
                    max_value = max_value.max(arr[j - 1]);
                }
            }
        }
        d[n]
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(
            Solution::max_sum_after_partitioning(vec![1, 15, 7, 9, 2, 5, 10], 3),
            84
        );
    }
}
