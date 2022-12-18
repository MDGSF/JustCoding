impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0i32; temperatures.len()];
        let mut stack = Vec::<usize>::new();
        for (i, &temperature) in temperatures.iter().enumerate() {
            while !stack.is_empty() && temperature > temperatures[stack[stack.len() - 1]] {
                result[stack[stack.len() - 1]] = (i - stack[stack.len() - 1]) as i32;
                stack.pop();
            }
            stack.push(i);
        }
        result
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );

        assert_eq!(
            Solution::daily_temperatures(vec![30, 40, 50, 60]),
            vec![1, 1, 1, 0]
        );

        assert_eq!(
            Solution::daily_temperatures(vec![30, 60, 90]),
            vec![1, 1, 0]
        );
    }
}
