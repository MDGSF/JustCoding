impl Solution {
  pub fn unique_paths_iii(mut grid: Vec<Vec<i32>>) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut start_row = 0;
    let mut start_col = 0;
    let mut end_row = 0;
    let mut end_col = 0;
    let mut empty = 1;
    let mut result = 0;

    for row in 0..rows {
      for col in 0..cols {
        if grid[row][col] == 0 {
          empty += 1;
        } else if grid[row][col] == 1 {
          start_row = row;
          start_col = col;
        } else if grid[row][col] == 2 {
          end_row = row;
          end_col = col;
        }
      }
    }

    Self::recursion(
      &mut grid,
      start_row,
      start_col,
      end_row,
      end_col,
      empty,
      &mut result,
    );

    result
  }

  fn recursion(
    grid: &mut Vec<Vec<i32>>,
    start_row: usize,
    start_col: usize,
    end_row: usize,
    end_col: usize,
    empty: usize,
    result: &mut i32,
  ) {
    if start_row >= grid.len() || start_col >= grid[0].len() || grid[start_row][start_col] < 0 {
      return;
    }
    if start_row == end_row && start_col == end_col {
      if empty == 0 {
        *result += 1;
      }
      return;
    }
    grid[start_row][start_col] = -2;

    Self::recursion(
      grid,
      start_row + 1,
      start_col,
      end_row,
      end_col,
      empty - 1,
      result,
    );

    if start_row > 0 {
      Self::recursion(
        grid,
        start_row - 1,
        start_col,
        end_row,
        end_col,
        empty - 1,
        result,
      );
    }

    Self::recursion(
      grid,
      start_row,
      start_col + 1,
      end_row,
      end_col,
      empty - 1,
      result,
    );

    if start_col > 0 {
      Self::recursion(
        grid,
        start_row,
        start_col - 1,
        end_row,
        end_col,
        empty - 1,
        result,
      );
    }

    grid[start_row][start_col] = 0;
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(
      Solution::unique_paths_iii(vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 2, -1],]),
      2
    );
  }
}
