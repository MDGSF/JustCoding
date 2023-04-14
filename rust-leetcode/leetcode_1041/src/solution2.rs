impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        let mut pos = (0, 0);
        let mut direction = 0; // 0:北 1:东 2:南 3:西
        let move_forward = vec![vec![0, 1], vec![1, 0], vec![0, -1], vec![-1, 0]];
        for c in instructions.chars() {
            if c == 'G' {
                pos.0 += move_forward[direction][0];
                pos.1 += move_forward[direction][1];
            } else if c == 'L' {
                direction = direction.wrapping_sub(1) % 4;
            } else if c == 'R' {
                direction = (direction + 1) % 4;
            }
        }
        (pos.0 == 0 && pos.1 == 0) || (direction != 0)
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
