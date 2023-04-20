impl Solution {
    pub fn delete_greatest_value(mut grid: Vec<Vec<i32>>) -> i32 {
        grid.iter_mut().for_each(|row| row.sort_unstable());
        (0..grid[0].len()).fold(0, |acc, col| {
            acc + (0..grid.len()).map(|row| grid[row][col]).max().unwrap()
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
