impl Solution {
  pub fn is_valid_sudoku(mut board: Vec<Vec<char>>) -> bool {
    let rows = board.len();
    let cols = board[0].len();
    for row in 0..rows {
      for col in 0..cols {
        if board[row][col] == '.' {
          continue;
        }
        let c = board[row][col];
        board[row][col] = '.';
        if !Self::is_valid(&board, row, col, c) {
          return false;
        }
        board[row][col] = c;
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
