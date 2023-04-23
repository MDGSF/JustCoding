use std::collections::HashSet;

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let code_len = 2i32.pow(n as u32) as usize;
        let mut code = vec![0; code_len];
        let mut set = HashSet::new();
        set.insert(0);
        for i in 1..code_len {
            let pre_num = code[i - 1];
            for j in 0..n {
                let num = pre_num ^ (1 << j);
                if !set.contains(&num) {
                    code[i] = num;
                    set.insert(num);
                    break;
                }
            }
        }
        code
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(Solution::gray_code(2), vec![0, 1, 3, 2]);
    }
}
