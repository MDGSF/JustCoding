use std::collections::BinaryHeap;

impl Solution {
    pub fn delete_greatest_value(grid: Vec<Vec<i32>>) -> i32 {
        let cols = grid[0].len();
        let mut heap_grid = grid.iter().fold(Vec::new(), |mut grid, row| {
            grid.push(row.iter().cloned().collect::<BinaryHeap<i32>>());
            grid
        });
        (0..cols).fold(0, |acc, _n| {
            acc + heap_grid
                .iter_mut()
                .map(|heap| heap.pop())
                .max()
                .unwrap()
                .unwrap()
        })
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let grid = vec![vec![1,2,4],vec![3,3,1]];
        assert_eq!(Solution::delete_greatest_value(grid), 8);
    }
}