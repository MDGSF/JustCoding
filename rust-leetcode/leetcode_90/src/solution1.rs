impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut cur = vec![];
        let mut result = vec![];
        let choose_pre = false;
        Self::recursion(&nums, 0, &mut cur, &mut result, choose_pre);
        result
    }

    fn recursion(
        nums: &Vec<i32>,
        first: usize,
        cur: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
        choose_pre: bool,
    ) {
        if first == nums.len() {
            result.push(cur.clone());
            return;
        }

        Self::recursion(nums, first + 1, cur, result, false);

        if !choose_pre && first > 0 && nums[first - 1] == nums[first] {
            return;
        }

        cur.push(nums[first]);
        Self::recursion(nums, first + 1, cur, result, true);
        cur.pop();
    }
}

pub struct Solution;
