use std::collections::VecDeque;

impl Solution {
  pub fn update_board(mut board: Vec<Vec<char>>, click: Vec<i32>) -> Vec<Vec<char>> {
    let mut queue = VecDeque::new();
    let click: Vec<usize> = vec![click[0] as usize, click[1] as usize];
    queue.push_back(click);
    while let Some(cur_click) = queue.pop_front() {
      let row = cur_click[0];
      let col = cur_click[1];
      if board[row][col] == 'M' {
        board[row][col] = 'X';
        break;
      } else if board[row][col] == 'E' {
        let mine_count = Self::find_adjoin_mine(&mut board, row, col);
        if mine_count == 0 {
          board[row][col] = 'B';
          let ms = Self::find_adjoin_e(&mut board, row, col);
          for c in ms.into_iter() {
            queue.push_back(c);
          }
        } else {
          board[row][col] = mine_count.to_string().chars().next().unwrap();
        }
      }
    }
    board
  }

  // 找到相邻的地雷的数量
  fn find_adjoin_mine(board: &mut Vec<Vec<char>>, row: usize, col: usize) -> usize {
    let directions = vec![
      vec![-1, 0],
      vec![1, 0],
      vec![0, -1],
      vec![0, 1],
      vec![-1, -1],
      vec![1, 1],
      vec![-1, 1],
      vec![1, -1],
    ];

    let mut count = 0;
    let rows = board.len();
    let cols = board[0].len();
    let old_row = row as isize;
    let old_col = col as isize;
    for direction in directions.iter() {
      let new_row = old_row + direction[0];
      let new_col = old_col + direction[1];
      if new_row >= 0
        && new_row < rows as isize
        && new_col >= 0
        && new_col < cols as isize
        && board[new_row as usize][new_col as usize] == 'M'
      {
        count += 1;
      }
    }
    count
  }

  // 找到所有相邻的'E'节点的坐标
  fn find_adjoin_e(board: &mut Vec<Vec<char>>, row: usize, col: usize) -> Vec<Vec<usize>> {
    let directions = vec![
      vec![-1, 0],
      vec![1, 0],
      vec![0, -1],
      vec![0, 1],
      vec![-1, -1],
      vec![1, 1],
      vec![-1, 1],
      vec![1, -1],
    ];

    let mut result = Vec::new();
    let rows = board.len();
    let cols = board[0].len();
    let old_row = row as isize;
    let old_col = col as isize;
    for direction in directions.iter() {
      let new_row = old_row + direction[0];
      let new_col = old_col + direction[1];
      if new_row >= 0
        && new_row < rows as isize
        && new_col >= 0
        && new_col < cols as isize
        && board[new_row as usize][new_col as usize] == 'E'
      {
        result.push(vec![new_row as usize, new_col as usize]);
      }
    }
    result
  }
}

pub struct Solution;
