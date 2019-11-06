/*
 * @lc app=leetcode.cn id=15 lang=javascript
 *
 * [15] 三数之和
 *
 * https://leetcode-cn.com/problems/3sum/description/
 *
 * algorithms
 * Medium (24.12%)
 * Likes:    1450
 * Dislikes: 0
 * Total Accepted:    108.1K
 * Total Submissions: 446.1K
 * Testcase Example:  '[-1,0,1,2,-1,-4]'
 *
 * 给定一个包含 n 个整数的数组 nums，判断 nums 中是否存在三个元素 a，b，c ，使得 a + b + c = 0
 * ？找出所有满足条件且不重复的三元组。
 *
 * 注意：答案中不可以包含重复的三元组。
 *
 * 例如, 给定数组 nums = [-1, 0, 1, 2, -1, -4]，
 *
 * 满足要求的三元组集合为：
 * [
 * ⁠ [-1, 0, 1],
 * ⁠ [-1, -1, 2]
 * ]
 *
 *
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {number[][]}
 */
var threeSum = function(nums) {
  let result = [];
  nums.sort((a, b) => a - b);
  for (let i = 0; i < nums.length; i += 1) {
    if (i > 0 && nums[i] === nums[i - 1]) {
      continue;
    }
    const target = -nums[i];
    const m = {};
    for (let j = i + 1; j < nums.length; j += 1) {
      const c = nums[j];
      const p = target - c;
      if (p in m) {
        result.push([nums[i], p, c]);
        delete m[p];
      } else {
        m[c] = j;
      }
    }
  }
  return result;
};
// @lc code=end

// [-1, 0, 1, 2, -1, -4]
// [ [-1, 0, 1], [-1, -1, 2] ]
const result = threeSum([-1, 0, 1, 2, -1, -4]);
console.log(result);
