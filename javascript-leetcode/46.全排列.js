/*
 * @lc app=leetcode.cn id=46 lang=javascript
 *
 * [46] 全排列
 *
 * https://leetcode-cn.com/problems/permutations/description/
 *
 * algorithms
 * Medium (72.16%)
 * Likes:    422
 * Dislikes: 0
 * Total Accepted:    52.9K
 * Total Submissions: 73.1K
 * Testcase Example:  '[1,2,3]'
 *
 * 给定一个没有重复数字的序列，返回其所有可能的全排列。
 *
 * 示例:
 *
 * 输入: [1,2,3]
 * 输出:
 * [
 * ⁠ [1,2,3],
 * ⁠ [1,3,2],
 * ⁠ [2,1,3],
 * ⁠ [2,3,1],
 * ⁠ [3,1,2],
 * ⁠ [3,2,1]
 * ]
 *
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {number[][]}
 */
var permute = function(nums) {
  let result = [];
  recursion(nums, 0, result);
  return result;
};

function recursion(nums, first, result) {
  if (first === nums.length) {
    result.push(nums.slice());
  }

  for (let i = first; i < nums.length; i += 1) {
    [nums[first], nums[i]] = [nums[i], nums[first]];
    recursion(nums, first + 1, result);
    [nums[first], nums[i]] = [nums[i], nums[first]];
  }
}

const result = permute([1, 2, 3]);
console.log(result);

// @lc code=end

