/*
 * @lc app=leetcode.cn id=74 lang=javascript
 *
 * [74] 搜索二维矩阵
 *
 * https://leetcode-cn.com/problems/search-a-2d-matrix/description/
 *
 * algorithms
 * Medium (35.93%)
 * Likes:    92
 * Dislikes: 0
 * Total Accepted:    19.2K
 * Total Submissions: 53.4K
 * Testcase Example:  '[[1,3,5,7],[10,11,16,20],[23,30,34,50]]\n3'
 *
 * 编写一个高效的算法来判断 m x n 矩阵中，是否存在一个目标值。该矩阵具有如下特性：
 *
 *
 * 每行中的整数从左到右按升序排列。
 * 每行的第一个整数大于前一行的最后一个整数。
 *
 *
 * 示例 1:
 *
 * 输入:
 * matrix = [
 * ⁠ [1,   3,  5,  7],
 * ⁠ [10, 11, 16, 20],
 * ⁠ [23, 30, 34, 50]
 * ]
 * target = 3
 * 输出: true
 *
 *
 * 示例 2:
 *
 * 输入:
 * matrix = [
 * ⁠ [1,   3,  5,  7],
 * ⁠ [10, 11, 16, 20],
 * ⁠ [23, 30, 34, 50]
 * ]
 * target = 13
 * 输出: false
 *
 */

// @lc code=start
/**
 * @param {number[][]} matrix
 * @param {number} target
 * @return {boolean}
 */
var searchMatrix = function(matrix, target) {
  let row = findLastLEInFirstCol(matrix, target);
  if (row === -1) {
    return false;
  }

  let cols =  matrix[0].length;
  let left = 0;
  let right = cols - 1;
  while (left <= right) {
    let mid = left + Math.floor((right - left) / 2);
    if (matrix[row][mid] === target) {
      return true;
    } else if (matrix[row][mid] < target) {
      left = mid + 1;
    } else {
      right = mid - 1;
    }
  }
  return false;
};

function findLastLEInFirstCol(matrix, target) {
  let rows = matrix.length;
  let left = 0;
  let right = rows - 1;
  while (left <= right) {
    let mid = left + Math.floor((right - left) / 2);
    if (matrix[mid][0] > target) {
      right = mid - 1;
    } else {
      if (mid === rows - 1 || matrix[mid+1][0] > target) {
        return mid;
      }
      left = mid + 1;
    }
  }
  return -1;
}
// @lc code=end

