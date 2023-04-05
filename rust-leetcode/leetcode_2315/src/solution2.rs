impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        s.split("|")
            .enumerate()
            .filter(|(index, _item)| index % 2 == 0)
            .map(|(_index, item)| item)
            .collect::<Vec<&str>>()
            .join("")
            .chars()
            .filter(|x| *x == '*')
            .count() as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(
            Solution::count_asterisks("l|*e*et|c**o|*de|".to_string()),
            2
        );
        assert_eq!(Solution::count_asterisks("iamprogrammer".to_string()), 0);
        assert_eq!(
            Solution::count_asterisks("yo|uar|e**|b|e***au|tifu|l".to_string()),
            5
        );
    }
}
