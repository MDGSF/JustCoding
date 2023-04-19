impl Solution {
    pub fn check_x_matrix(grid: Vec<Vec<i32>>) -> bool {
        let n = grid.len();
        for row in 0..n {
            for col in 0..n {
                if row == col || row + col == n - 1 {
                    if grid[row][col] == 0 {
                        return false;
                    }
                } else {
                    if grid[row][col] != 0 {
                        return false;
                    }
                }
            }
        }
        true
    }
}

pub struct Solution;
