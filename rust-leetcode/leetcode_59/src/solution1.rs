impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let is_valid_position = |x, y, matrix: &Vec<Vec<i32>>| -> bool {
            x >= 0 && y >= 0 && x < n && y < n && matrix[x as usize][y as usize] == 0
        };

        let directions = vec![vec![-1, 0], vec![0, 1], vec![1, 0], vec![0, -1]];
        let mut dir: usize = 1;
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut result = vec![vec![0; n as usize]; n as usize];

        for i in 1..=n * n {
            result[x as usize][y as usize] = i;
            let new_x = x + directions[dir][0];
            let new_y = y + directions[dir][1];
            if is_valid_position(new_x, new_y, &result) {
                x = new_x;
                y = new_y;
            } else {
                dir = (dir + 1) % 4;
                x = x + directions[dir][0];
                y = y + directions[dir][1];
            }
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
        assert_eq!(
            Solution::generate_matrix(3),
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
        );
    }
}
