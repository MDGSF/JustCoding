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
  nums.sort((a, b) => a - b);
  const n = nums.length;
  let minDiff = 99999999;
  let minSum = 0;
  for (let i = 0; i < n; i += 1) {
    if (i > 0 && nums[i] === nums[i - 1]) {
      continue;
    }
    let [start, end] = [i + 1, n - 1];
    while (start < end) {
      const sum = nums[start] + nums[end] + nums[i];
      if (sum === target) {
        return sum;
      } else {
        const curDiff = sum - target;
        const curAbsDiff = Math.abs(curDiff);
        if (curAbsDiff < minDiff) {
          minDiff = curAbsDiff;
          minSum = sum;
        }

        if (curDiff < 0) {
          while (nums[start] === nums[start + 1]) {
            start += 1;
          }
          start += 1;
        } else {
          while (nums[end] === nums[end - 1]) {
            end -= 1;
          }
          end -= 1;
        }
      }
    }
  }
  return minSum;
};
// @lc code=end

const result = threeSumClosest([-1, 2, 1, -4], 1);
console.log(result);
