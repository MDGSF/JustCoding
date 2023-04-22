impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let first_row_has_zero = matrix[0].iter().any(|&num| num == 0);
        let first_col_has_zero = matrix.iter().any(|row| row[0] == 0);

        for row in 1..rows {
            for col in 1..cols {
                if matrix[row][col] == 0 {
                    matrix[row][0] = 0;
                    matrix[0][col] = 0;
                }
            }
        }

        for row in 1..rows {
            for col in 1..cols {
                if matrix[row][0] == 0 || matrix[0][col] == 0 {
                    matrix[row][col] = 0;
                }
            }
        }

        if first_row_has_zero {
            matrix[0].iter_mut().for_each(|num| *num = 0);
        }

        if first_col_has_zero {
            matrix.iter_mut().for_each(|row| row[0] = 0);
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]);
    }

    #[test]
    fn test02() {
        let mut matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(
            matrix,
            vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]]
        );
    }
}
