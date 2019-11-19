/*
 * @lc app=leetcode.cn id=18 lang=javascript
 *
 * [18] 四数之和
 *
 * https://leetcode-cn.com/problems/4sum/description/
 *
 * algorithms
 * Medium (35.96%)
 * Likes:    298
 * Dislikes: 0
 * Total Accepted:    36.4K
 * Total Submissions: 101.1K
 * Testcase Example:  '[1,0,-1,0,-2,2]\n0'
 *
 * 给定一个包含 n 个整数的数组 nums 和一个目标值 target，判断 nums 中是否存在四个元素 a，b，c 和 d ，使得 a + b + c
 * + d 的值与 target 相等？找出所有满足条件且不重复的四元组。
 *
 * 注意：
 *
 * 答案中不可以包含重复的四元组。
 *
 * 示例：
 *
 * 给定数组 nums = [1, 0, -1, 0, -2, 2]，和 target = 0。
 *
 * 满足要求的四元组集合为：
 * [
 * ⁠ [-1,  0, 0, 1],
 * ⁠ [-2, -1, 1, 2],
 * ⁠ [-2,  0, 0, 2]
 * ]
 *
 *
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number[][]}
 */
var fourSum = function(nums, target) {
  return FindSum(nums, target, 4);
};
// @lc code=end

function FindSum(nums, target, N) {
  let result = [];
  nums.sort((a, b) => a - b);
  findSum(nums, 0, nums.length - 1, target, N, [], result);
  return result;
}

function findSum(nums, start, end, target, N, prefix, result) {
  if (nums.length < N || end - start + 1 < N) {
    return;
  }

  if (N === 2) {
    twoSum(nums, start, end, target, prefix, result);
    return;
  }

  for (let i = start; i <= end; i += 1) {
    if (i > start && nums[i] === nums[i - 1]) {
      continue;
    }
    const subtarget = target - nums[i];
    prefix.push(nums[i]);
    findSum(nums, i + 1, end, subtarget, N - 1, prefix, result);
    prefix.pop();
  }
}

function twoSum(nums, start, end, target, prefix, result) {
  while (start < end) {
    const sum = nums[start] + nums[end];
    if (sum < target) {
      start += 1;
    } else if (sum > target) {
      end -= 1;
    } else {
      result.push(prefix.concat([nums[start], nums[end]]));

      while (nums[start] === nums[start + 1]) {
        start += 1;
      }
      start += 1;

      while (nums[end] === nums[end - 1]) {
        end -= 1;
      }
      end -= 1;
    }
  }
}

// const nums = [1, 0, -1, 0, -2, 2];
// const target = 0;
const nums = [-1, 0, -5, -2, -2, -4, 0, 1, -2];
const target = -9;
const result = fourSum(nums, target);
console.log(result);
