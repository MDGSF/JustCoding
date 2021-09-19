impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        Self::solve(board);
    }

    fn solve(board: &mut Vec<Vec<char>>) -> bool {
        let rows = board.len();
        let cols = board[0].len();
        for row in 0..rows {
            for col in 0..cols {
                if board[row][col] != '.' {
                    continue;
                } else {
                    for &num in ['1', '2', '3', '4', '5', '6', '7', '8', '9'].iter() {
                        if Self::is_valid(board, row, col, num) {
                            board[row][col] = num;
                            if Self::solve(board) {
                                return true;
                            }
                            board[row][col] = '.';
                        }
                    }
                    return false;
                }
            }
        }
        true
    }

    fn is_valid(board: &Vec<Vec<char>>, row: usize, col: usize, c: char) -> bool {
        for i in 0..9 {
            if board[i][col] == c {
                // 同一列
                return false;
            }
            if board[row][i] == c {
                // 同一行
                return false;
            }
            if board[row / 3 * 3 + i / 3][col / 3 * 3 + i % 3] == c {
                // 同一个3x3方格内
                return false;
            }
        }
        true
    }
}

pub struct Solution;
