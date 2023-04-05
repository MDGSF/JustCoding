impl Solution {
    pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
        let mut real_distance = vec![usize::MAX; 26];
        for (i, b) in s.bytes().enumerate() {
            let idx = (b - b'a') as usize;
            if real_distance[idx] == usize::MAX {
                real_distance[idx] = i;
            } else {
                real_distance[idx] = i - real_distance[idx] - 1;
            }
        }

        for (i, _n) in real_distance.iter().enumerate() {
            if real_distance[i] != usize::MAX {
                if real_distance[i] != distance[i] as usize {
                    return false;
                }
            }
        }

        true
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let s = "abaccb".to_string();
        let distance = vec![
            1, 3, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let result = Solution::check_distances(s, distance);
        assert_eq!(result, true);
    }
}
