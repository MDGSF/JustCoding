/*
 * @lc app=leetcode.cn id=79 lang=javascript
 *
 * [79] 单词搜索
 *
 * https://leetcode-cn.com/problems/word-search/description/
 *
 * algorithms
 * Medium (39.18%)
 * Likes:    228
 * Dislikes: 0
 * Total Accepted:    23.2K
 * Total Submissions: 59.2K
 * Testcase Example:  '[["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]]\n"ABCCED"'
 *
 * 给定一个二维网格和一个单词，找出该单词是否存在于网格中。
 *
 * 单词必须按照字母顺序，通过相邻的单元格内的字母构成，其中“相邻”单元格是那些水平相邻或垂直相邻的单元格。同一个单元格内的字母不允许被重复使用。
 *
 * 示例:
 *
 * board =
 * [
 * ⁠ ['A','B','C','E'],
 * ⁠ ['S','F','C','S'],
 * ⁠ ['A','D','E','E']
 * ]
 *
 * 给定 word = "ABCCED", 返回 true.
 * 给定 word = "SEE", 返回 true.
 * 给定 word = "ABCB", 返回 false.
 *
 */

// @lc code=start
/**
 * @param {character[][]} board
 * @param {string} word
 * @return {boolean}
 */
var exist = function(board, word) {
  const visited = Array(board.length)
    .fill(null)
    .map(() => {
      return Array(board[0].length).fill(false);
    });
  for (let i = 0; i < board.length; i += 1) {
    for (let j = 0; j < board[0].length; j += 1) {
      if (dfs(board, i, j, word, visited)) {
        return true;
      }
    }
  }
  return false;
};

const directions = [[1, 0], [-1, 0], [0, 1], [0, -1]];

function dfs(board, row, col, word, visited) {
  if (word.length === 0) {
    return true;
  }
  if (board[row][col] !== word.charAt(0)) {
    return false;
  }
  if (word.length === 1) {
    return true;
  }
  visited[row][col] = true;
  for (let i = 0; i < directions.length; i += 1) {
    const newrow = row + directions[i][0];
    const newcol = col + directions[i][1];
    if (
      newrow >= 0 &&
      newrow < board.length &&
      newcol >= 0 &&
      newcol < board[0].length &&
      !visited[newrow][newcol]
    ) {
      if (dfs(board, newrow, newcol, word.substring(1), visited)) {
        return true;
      }
    }
  }
  visited[row][col] = false;
  return false;
}

const board = [
  ['A', 'B', 'C', 'E'],
  ['S', 'F', 'C', 'S'],
  ['A', 'D', 'E', 'E'],
];
const word = 'ABCB';
const result = exist(board, word);
console.log(result);
// @lc code=end
