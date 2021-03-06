/*
 * @lc app=leetcode.cn id=16 lang=javascript
 *
 * [16] 最接近的三数之和
 *
 * https://leetcode-cn.com/problems/3sum-closest/description/
 *
 * algorithms
 * Medium (41.74%)
 * Likes:    278
 * Dislikes: 0
 * Total Accepted:    49.9K
 * Total Submissions: 119.2K
 * Testcase Example:  '[-1,2,1,-4]\n1'
 *
 * 给定一个包括 n 个整数的数组 nums 和 一个目标值 target。找出 nums 中的三个整数，使得它们的和与 target
 * 最接近。返回这三个数的和。假定每组输入只存在唯一答案。
 *
 * 例如，给定数组 nums = [-1，2，1，-4], 和 target = 1.
 *
 * 与 target 最接近的三个数的和为 2. (-1 + 2 + 1 = 2).
 *
 *
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number}
 */
var threeSumClosest = function(nums, target) {
  let closestSum = nums[0] + nums[1] + nums[2],
    smallestDiff = Math.abs(target - closestSum);

  for (let i = 0; i < nums.length; i++) {
    for (let j = i + 1; j < nums.length; j++) {
      for (let k = j + 1; k < nums.length; k++) {
        let setSum = nums[i] + nums[j] + nums[k];
        if (setSum == target) {
          return setSum;
        }

        let diff = Math.abs(target - setSum);
        if (diff < smallestDiff) {
          closestSum = setSum;
          smallestDiff = diff;
        }
      }
    }
  }

  return closestSum;
};
// @lc code=end

const result = threeSumClosest([-1, 2, 1, -4], 1);
console.log(result);
