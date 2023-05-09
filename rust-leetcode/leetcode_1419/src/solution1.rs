impl Solution {
    pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
        if croak_of_frogs.len() % 5 != 0 {
            return -1;
        }

        let mut c = 0;
        let mut r = 0;
        let mut o = 0;
        let mut a = 0;
        let mut k = 0;

        let is_desc = |c: i32, r: i32, o: i32, a: i32, k: i32| c >= r && r >= o && o >= a && a >= k;

        let is_the_same =
            |c: i32, r: i32, o: i32, a: i32, k: i32| c == r && r == o && o == a && a == k;

        let mut max_value = 0;

        let croak_of_frogs = croak_of_frogs.as_bytes();
        for &b in croak_of_frogs.iter() {
            if b == b'c' {
                c += 1;
            } else if b == b'r' {
                r += 1;
            } else if b == b'o' {
                o += 1;
            } else if b == b'a' {
                a += 1;
            } else {
                k += 1;
            }

            if !is_desc(c, r, o, a, k) {
                return -1;
            }

            if b == b'k' {
                max_value = max_value.max(c);
                c -= k;
                r -= k;
                o -= k;
                a -= k;
                k -= k;
            }
        }

        if !is_the_same(c, r, o, a, k) {
            return -1;
        }

        max_value
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(Solution::min_number_of_frogs("croakcroak".to_string()), 1);
    }

    #[test]
    fn test02() {
        assert_eq!(Solution::min_number_of_frogs("crcoakroak".to_string()), 2);
    }

    #[test]
    fn test03() {
        assert_eq!(Solution::min_number_of_frogs("croakcrook".to_string()), -1);
    }

    #[test]
    fn test04() {
        assert_eq!(Solution::min_number_of_frogs("croakcroa".to_string()), -1);
    }

    #[test]
    fn test05() {
        assert_eq!(
            Solution::min_number_of_frogs("crocakcroraoakk".to_string()),
            2
        );
    }

    #[test]
    fn test06() {
        assert_eq!(
            Solution::min_number_of_frogs("cccccccrrooaakk".to_string()),
            -1
        );
    }
}
