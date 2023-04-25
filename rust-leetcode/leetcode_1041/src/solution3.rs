impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        let dirs = vec![vec![0, 1], vec![1, 0], vec![0, -1], vec![-1, 0]];
        instructions
            .chars()
            .fold(Some((0, 0, 0)), |acc, c| match (acc, c) {
                (Some((x, y, i)), 'G') => Some((x + dirs[i][0], y + dirs[i][1], i)),
                (Some((x, y, i)), 'L') => Some((x, y, (i + 3) % 4)),
                (Some((x, y, i)), 'R') => Some((x, y, (i + 1) % 4)),
                _ => unreachable!(),
            })
            .map(|(x, y, i)| (x == 0 && y == 0) || (i != 0))
            .unwrap()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(Solution::is_robot_bounded("GGLLGG".to_string()), true);
        assert_eq!(Solution::is_robot_bounded("GG".to_string()), false);
        assert_eq!(Solution::is_robot_bounded("GL".to_string()), true);
    }
}
