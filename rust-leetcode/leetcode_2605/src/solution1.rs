use std::collections::BTreeSet;

impl Solution {
    pub fn min_number(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = 10;
        let map = nums1
            .iter()
            .chain(nums2.iter())
            .fold(vec![0; n], |mut map, &num| {
                map[num as usize] += 1;
                map
            });
        let set1 = nums1.iter().cloned().collect::<BTreeSet<i32>>();
        let set2 = nums2.iter().cloned().collect::<BTreeSet<i32>>();

        for i in 0..n {
            if map[i] >= 2 {
                return i as i32;
            }
        }

        let mut first: i32 = 0;
        for i in 0..n {
            if map[i] == 1 {
                first = i as i32;
                break;
            }
        }

        let second: i32 = if set1.contains(&first) {
            *set2.iter().next().unwrap()
        } else {
            *set1.iter().next().unwrap()
        };

        first * 10 + second
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(Solution::min_number(vec![4, 1, 3], vec![5, 7]), 15);
    }
}
