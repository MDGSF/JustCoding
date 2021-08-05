use std::collections::VecDeque;

impl Solution {
  pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
    if grid.is_empty() {
      return 0;
    }
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;
    for row in 0..rows {
      for col in 0..cols {
        if grid[row][col] == '1' {
          count += 1;
          Self::destory_island_bfs(&mut grid, row, col);
        }
      }
    }
    count
  }

  fn destory_island_bfs(grid: &mut Vec<Vec<char>>, row: usize, col: usize) {
    let directions = vec![vec![-1, 0], vec![1, 0], vec![0, -1], vec![0, 1]];
    let rows = grid.len();
    let cols = grid[0].len();
    grid[row][col] = '0';
    let mut queue = VecDeque::new();
    queue.push_back(vec![row, col]);
    while let Some(node) = queue.pop_front() {
      let old_row = node[0] as isize;
      let old_col = node[1] as isize;
      for direction in directions.iter() {
        let new_row = old_row + direction[0];
        let new_col = old_col + direction[1];
        if new_row >= 0
          && new_row < rows as isize
          && new_col >= 0
          && new_col < cols as isize
          && grid[new_row as usize][new_col as usize] == '1'
        {
          let new_row = new_row as usize;
          let new_col = new_col as usize;
          grid[new_row][new_col] = '0';
          queue.push_back(vec![new_row, new_col]);
        }
      }
    }
  }
}

pub struct Solution;
