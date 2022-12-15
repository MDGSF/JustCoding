impl Solution {
    pub fn get_lucky(s: String, k: i32) -> i32 {
        let numstr = s.chars().fold(String::new(), |mut acc, c| {
            let num = (c as usize - 97 + 1).to_string();
            acc.push_str(&num);
            acc
        });

        (0..k)
            .fold(numstr, |numstr, _| {
                numstr
                    .chars()
                    .fold(0, |acc, x| acc + (x as usize - 64))
                    .to_string()
            })
            .parse::<i32>()
            .unwrap()
    }
}

pub struct Solution;

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(Solution::get_lucky("iiii", 1), 36);
        assert_eq!(Solution::get_lucky("leetcode", 2), 6);
        assert_eq!(Solution::get_lucky("zbax", 2), 8);
    }
}
