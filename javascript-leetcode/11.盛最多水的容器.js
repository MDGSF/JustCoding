/*
 * @lc app=leetcode.cn id=11 lang=javascript
 *
 * [11] 盛最多水的容器
 *
 * https://leetcode-cn.com/problems/container-with-most-water/description/
 *
 * algorithms
 * Medium (58.29%)
 * Likes:    839
 * Dislikes: 0
 * Total Accepted:    88K
 * Total Submissions: 150.9K
 * Testcase Example:  '[1,8,6,2,5,4,8,3,7]'
 *
 * 给定 n 个非负整数 a1，a2，...，an，每个数代表坐标中的一个点 (i, ai) 。在坐标内画 n 条垂直线，垂直线 i 的两个端点分别为
 * (i, ai) 和 (i, 0)。找出其中的两条线，使得它们与 x 轴共同构成的容器可以容纳最多的水。
 *
 * 说明：你不能倾斜容器，且 n 的值至少为 2。
 *
 *
 *
 * 图中垂直线代表输入数组 [1,8,6,2,5,4,8,3,7]。在此情况下，容器能够容纳水（表示为蓝色部分）的最大值为 49。
 *
 *
 *
 * 示例:
 *
 * 输入: [1,8,6,2,5,4,8,3,7]
 * 输出: 49
 *
 */

// @lc code=start
/**
 * @param {number[]} height
 * @return {number}
 */
var maxArea = function(height) {
  function min(a, b) {
    return a < b ? a : b;
  }

  function max(a, b) {
    return a > b ? a : b;
  }

  const F = [];
  F[0] = 0;
  F[1] = min(height[0], height[1]) * 1;

  for (let i = 2; i < height.length; i += 1) {
    const sub1 = F[i - 1];

    let sub2MaxArea = 0;
    for (let j = 0; j < i; j += 1) {
      const h = min(height[j], height[i]);
      const w = i - j;
      const area = h * w;
      if (area > sub2MaxArea) {
        sub2MaxArea = area;
      }
    }

    F[i] = max(sub1, sub2MaxArea);
  }
  return F[height.length - 1];
};
// @lc code=end
