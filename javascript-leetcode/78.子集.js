/*
 * @lc app=leetcode.cn id=78 lang=javascript
 *
 * [78] 子集
 *
 * https://leetcode-cn.com/problems/subsets/description/
 *
 * algorithms
 * Medium (74.95%)
 * Likes:    366
 * Dislikes: 0
 * Total Accepted:    41.5K
 * Total Submissions: 55.3K
 * Testcase Example:  '[1,2,3]'
 *
 * 给定一组不含重复元素的整数数组 nums，返回该数组所有可能的子集（幂集）。
 *
 * 说明：解集不能包含重复的子集。
 *
 * 示例:
 *
 * 输入: nums = [1,2,3]
 * 输出:
 * [
 * ⁠ [3],
 * [1],
 * [2],
 * [1,2,3],
 * [1,3],
 * [2,3],
 * [1,2],
 * []
 * ]
 *
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {number[][]}
 */
var subsets = function(nums) {
  let result = [];
  const size = nums.length;
  const count = 1 << size;
  for (let i = 0; i < count; i += 1) {
    const tmp = [];
    for (let j = 0; j < size; j += 1) {
      if ((i & (1 << j)) !== 0) {
        tmp.push(nums[j]);
      }
    }
    result.push(tmp);
  }
  return result;
};

const result = subsets([1, 2, 3]);
console.log(result);
// @lc code=end

