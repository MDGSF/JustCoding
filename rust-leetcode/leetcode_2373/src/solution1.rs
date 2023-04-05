impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        let mut max_local = vec![vec![0; n - 2]; n - 2];
        for row in 0..(n - 2) {
            for col in 0..(n - 2) {
                let mut local_max_value = grid[row][col];
                for local_row in 0..3 {
                    for local_col in 0..3 {
                        let cur_row = row + local_row;
                        let cur_col = col + local_col;
                        local_max_value = local_max_value.max(grid[cur_row][cur_col]);
                    }
                }
                max_local[row][col] = local_max_value;
            }
        }
        max_local
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let grid = vec![
            vec![9, 9, 8, 1],
            vec![5, 6, 2, 6],
            vec![8, 2, 6, 4],
            vec![6, 2, 2, 2],
        ];
        let expected_result = vec![vec![9, 9], vec![8, 6]];
        assert_eq!(Solution::largest_local(grid), expected_result);
    }

    #[test]
    fn test02() {
        let grid = vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 2, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
        ];
        let expected_result = vec![vec![2, 2, 2], vec![2, 2, 2], vec![2, 2, 2]];
        assert_eq!(Solution::largest_local(grid), expected_result);
    }
}
