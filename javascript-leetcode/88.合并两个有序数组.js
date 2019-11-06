/*
 * @lc app=leetcode.cn id=88 lang=javascript
 *
 * [88] 合并两个有序数组
 *
 * https://leetcode-cn.com/problems/merge-sorted-array/description/
 *
 * algorithms
 * Easy (45.52%)
 * Likes:    331
 * Dislikes: 0
 * Total Accepted:    78.3K
 * Total Submissions: 171.9K
 * Testcase Example:  '[1,2,3,0,0,0]\n3\n[2,5,6]\n3'
 *
 * 给定两个有序整数数组 nums1 和 nums2，将 nums2 合并到 nums1 中，使得 num1 成为一个有序数组。
 *
 * 说明:
 *
 *
 * 初始化 nums1 和 nums2 的元素数量分别为 m 和 n。
 * 你可以假设 nums1 有足够的空间（空间大小大于或等于 m + n）来保存 nums2 中的元素。
 *
 *
 * 示例:
 *
 * 输入:
 * nums1 = [1,2,3,0,0,0], m = 3
 * nums2 = [2,5,6],       n = 3
 *
 * 输出: [1,2,2,3,5,6]
 *
 */

// @lc code=start
/**
 * @param {number[]} nums1
 * @param {number} m
 * @param {number[]} nums2
 * @param {number} n
 * @return {void} Do not return anything, modify nums1 in-place instead.
 */
var merge = function(nums1, m, nums2, n) {
  const size = m + n;
  let curIdx = size - 1;
  let idx1 = m - 1;
  let idx2 = n - 1;
  while (idx1 >= 0 && idx2 >= 0) {
    if (nums1[idx1] > nums2[idx2]) {
      nums1[curIdx] = nums1[idx1];
      idx1 -= 1;
    } else {
      nums1[curIdx] = nums2[idx2];
      idx2 -= 1;
    }
    curIdx -= 1;
  }

  while (idx1 >= 0) {
    nums1[curIdx--] = nums1[idx1--];
  }

  while (idx2 >= 0) {
    nums1[curIdx--] = nums2[idx2--];
  }
};
// @lc code=end
