use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
  pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    if grid.is_empty() {
      return 0;
    }
    let rows = grid.len();
    let cols = grid[0].len();

    let mut p = HashMap::new();
    for row in 0..rows {
      for col in 0..cols {
        if grid[row][col] == '1' {
          p.insert(row * cols + col, row * cols + col);
        }
      }
    }

    let directions = vec![vec![-1, 0], vec![1, 0], vec![0, -1], vec![0, 1]];
    for row in 0..rows {
      for col in 0..cols {
        if grid[row][col] == '1' {
          let old_row = row as isize;
          let old_col = col as isize;
          for direction in directions.iter() {
            let new_row = old_row + direction[0];
            let new_col = old_col + direction[1];
            if new_row >= 0
              && new_row < rows as isize
              && new_col >= 0
              && new_col < cols as isize
              && grid[new_row as usize][new_col as usize] == '1'
            {
              Self::union(
                &mut p,
                row * cols + col,
                new_row as usize * rows + new_col as usize,
              );
            }
          }
        }
      }
    }

    let roots: Vec<&usize> = p.keys().collect();
    let sets: HashSet<&usize> = HashSet::from_iter(roots.iter().cloned());
    sets.len() as i32
  }

  fn union(p: &mut HashMap<usize, usize>, i: usize, j: usize) {
    let p1 = Self::parent(p, i);
    let p2 = Self::parent(p, j);
    p.insert(p1, p2);
  }

  fn parent(p: &mut HashMap<usize, usize>, mut i: usize) -> usize {
    let mut root = i;
    while p.get(&root) != Some(&root) {
      root = *p.get(&root).unwrap();
    }
    while p.get(&i) != Some(&i) {
      let x = i;
      i = *p.get(&i).unwrap();
      p.insert(x, root);
    }
    root
  }
}

pub struct Solution;
