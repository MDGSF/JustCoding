impl Solution {
    pub fn simplify_path(path: String) -> String {
        let parts = path
            .split('/')
            .filter(|&part| !part.is_empty() && part != ".")
            .collect::<Vec<_>>();

        let mut stack = vec![];
        for &part in parts.iter() {
            if part == ".." {
                if !stack.is_empty() {
                    stack.pop();
                }
            } else {
                stack.push(part);
            }
        }

        "/".to_string() + &stack.join("/")
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(
            Solution::simplify_path("/home/".to_string()),
            "/home".to_string()
        );
    }

    #[test]
    fn test02() {
        assert_eq!(Solution::simplify_path("/../".to_string()), "/".to_string());
    }

    #[test]
    fn test03() {
        assert_eq!(
            Solution::simplify_path("/home//foo/".to_string()),
            "/home/foo".to_string()
        );
    }

    #[test]
    fn test04() {
        assert_eq!(
            Solution::simplify_path("/a/./b/../../c/".to_string()),
            "/c".to_string()
        );
    }
}
