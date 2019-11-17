/*
 * @lc app=leetcode.cn id=54 lang=javascript
 *
 * [54] 螺旋矩阵
 *
 * https://leetcode-cn.com/problems/spiral-matrix/description/
 *
 * algorithms
 * Medium (37.21%)
 * Likes:    230
 * Dislikes: 0
 * Total Accepted:    27.7K
 * Total Submissions: 74.1K
 * Testcase Example:  '[[1,2,3],[4,5,6],[7,8,9]]'
 *
 * 给定一个包含 m x n 个元素的矩阵（m 行, n 列），请按照顺时针螺旋顺序，返回矩阵中的所有元素。
 *
 * 示例 1:
 *
 * 输入:
 * [
 * ⁠[ 1, 2, 3 ],
 * ⁠[ 4, 5, 6 ],
 * ⁠[ 7, 8, 9 ]
 * ]
 * 输出: [1,2,3,6,9,8,7,4,5]
 *
 *
 * 示例 2:
 *
 * 输入:
 * [
 * ⁠ [1, 2, 3, 4],
 * ⁠ [5, 6, 7, 8],
 * ⁠ [9,10,11,12]
 * ]
 * 输出: [1,2,3,4,8,12,11,10,9,5,6,7]
 *
 *
 */

// @lc code=start
/**
 * @param {number[][]} matrix
 * @return {number[]}
 */
var spiralOrder = function(matrix) {
  if (matrix.length === 0) {
    return [];
  }

  let result = [];
  const rows = matrix.length;
  const columns = matrix[0].length;
  let start = 0;
  while (rows > start * 2 && columns > start * 2) {
    PrintMatrixInCircle(matrix, start, result);
    start += 1;
  }
  return result;
};

// PrintMatrixInCircle 打印出最外面一圈
// start = 0 表示第一圈
// start = 1 表示第二圈
// ...
function PrintMatrixInCircle(matrix, start, result) {
  let endX = matrix[0].length - 1 - start;
  let endY = matrix.length - 1 - start;

  // 从左到右
  for (let i = start; i <= endX; i += 1) {
    let number = matrix[start][i];
    result.push(number);
  }

  // 从上到下
  if (start < endY) {
    for (let i = start + 1; i <= endY; i += 1) {
      let number = matrix[i][endX];
      result.push(number);
    }
  }

  // 从右到左
  if (start < endX && start < endY) {
    for (let i = endX - 1; i >= start; i -= 1) {
      let number = matrix[endY][i];
      result.push(number);
    }
  }

  // 从下到上
  if (start < endX && start < endY - 1) {
    for (let i = endY - 1; i >= start + 1; i -= 1) {
      let number = matrix[i][start];
      result.push(number);
    }
  }
}
// @lc code=end
