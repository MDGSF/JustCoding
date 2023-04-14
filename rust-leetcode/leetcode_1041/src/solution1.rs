impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        let mut robot_x = 0;
        let mut robot_y = 0;
        let mut robot_direction = 0; // 0:北 1:东 2:南 3:西
        let move_forward = vec![
            vec![0, 1],  // 北
            vec![1, 0],  // 东
            vec![0, -1], // 南
            vec![-1, 0], // 西
        ];
        for c in instructions.chars() {
            match c {
                'G' => {
                    robot_x += move_forward[robot_direction][0];
                    robot_y += move_forward[robot_direction][1];
                }
                'L' => {
                    if robot_direction > 0 {
                        robot_direction -= 1;
                    } else {
                        robot_direction = 3;
                    }
                }
                'R' => {
                    if robot_direction < 3 {
                        robot_direction += 1;
                    } else {
                        robot_direction = 0;
                    }
                }
                _ => {}
            }
        }

        if robot_x == 0 && robot_y == 0 {
            return true;
        }
        if robot_direction != 0 {
            return true;
        }

        false
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
