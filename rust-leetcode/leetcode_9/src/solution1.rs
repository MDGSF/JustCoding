impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut num = x;
        let mut reverse_x = 0;
        while num > 0 {
            let t = num % 10;
            reverse_x = reverse_x * 10 + t;
            num = num / 10;
        }

        reverse_x == x
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(-121), false);
        assert_eq!(Solution::is_palindrome(10), false);
    }
}
