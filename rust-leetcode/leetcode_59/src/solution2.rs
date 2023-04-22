impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut left: i32 = 0;
        let mut right: i32 = n - 1;
        let mut top: i32 = 0;
        let mut bottom: i32 = n - 1;
        let mut mat = vec![vec![0; n as usize]; n as usize];
        let mut num = 1;
        while num <= n * n {
            // left to right
            for col in left..=right {
                mat[top as usize][col as usize] = num;
                num += 1;
            }
            top += 1;

            // top to bottom
            for row in top..=bottom {
                mat[row as usize][right as usize] = num;
                num += 1;
            }
            right -= 1;

            // right to left
            for col in (left..=right).rev() {
                mat[bottom as usize][col as usize] = num;
                num += 1;
            }
            bottom -= 1;

            // bottom to top
            for row in (top..=bottom).rev() {
                mat[row as usize][left as usize] = num;
                num += 1;
            }
            left += 1;
        }
        mat
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(
            Solution::generate_matrix(3),
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
        );
    }

    #[test]
    fn test02() {
        assert_eq!(Solution::generate_matrix(1), vec![vec![1]]);
    }
}
