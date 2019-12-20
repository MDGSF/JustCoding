/*
 * @lc app=leetcode.cn id=47 lang=javascript
 *
 * [47] 全排列 II
 *
 * https://leetcode-cn.com/problems/permutations-ii/description/
 *
 * algorithms
 * Medium (54.56%)
 * Likes:    173
 * Dislikes: 0
 * Total Accepted:    25.8K
 * Total Submissions: 47K
 * Testcase Example:  '[1,1,2]'
 *
 * 给定一个可包含重复数字的序列，返回所有不重复的全排列。
 *
 * 示例:
 *
 * 输入: [1,1,2]
 * 输出:
 * [
 * ⁠ [1,1,2],
 * ⁠ [1,2,1],
 * ⁠ [2,1,1]
 * ]
 *
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {number[][]}
 */
var permuteUnique = function(nums) {
  let result = [];
  let m = counter(nums);
  recursion(nums, m, [], result);
  return result;
};

function recursion(nums, m, cur, result) {
  if (cur.length === nums.length) {
    result.push(cur.map(e => +e));
    return;
  }

  for (let num in m) {
    if (m[num] > 0) {
      cur.push(num);
      m[num] -= 1;

      recursion(nums, m, cur, result);

      cur.pop();
      m[num] += 1;
    }
  }
}

function counter(nums) {
  const m = {};
  for (let i = 0; i < nums.length; i += 1) {
    const num = nums[i];
    if (num in m) {
      m[num] += 1;
    } else {
      m[num] = 1;
    }
  }
  return m;
}

//const nums = [1, 1, 2];
const nums = [2, 2, 1, 1];
const result = permuteUnique(nums);
console.log(result);

// @lc code=end

