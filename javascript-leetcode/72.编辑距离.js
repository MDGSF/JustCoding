/*
 * @lc app=leetcode.cn id=72 lang=javascript
 *
 * [72] 编辑距离
 *
 * https://leetcode-cn.com/problems/edit-distance/description/
 *
 * algorithms
 * Hard (54.77%)
 * Likes:    375
 * Dislikes: 0
 * Total Accepted:    17.4K
 * Total Submissions: 31.6K
 * Testcase Example:  '"horse"\n"ros"'
 *
 * 给定两个单词 word1 和 word2，计算出将 word1 转换成 word2 所使用的最少操作数 。
 *
 * 你可以对一个单词进行如下三种操作：
 *
 *
 * 插入一个字符
 * 删除一个字符
 * 替换一个字符
 *
 *
 * 示例 1:
 *
 * 输入: word1 = "horse", word2 = "ros"
 * 输出: 3
 * 解释:
 * horse -> rorse (将 'h' 替换为 'r')
 * rorse -> rose (删除 'r')
 * rose -> ros (删除 'e')
 *
 *
 * 示例 2:
 *
 * 输入: word1 = "intention", word2 = "execution"
 * 输出: 5
 * 解释:
 * intention -> inention (删除 't')
 * inention -> enention (将 'i' 替换为 'e')
 * enention -> exention (将 'n' 替换为 'x')
 * exention -> exection (将 'n' 替换为 'c')
 * exection -> execution (插入 'u')
 *
 *
 */

// @lc code=start
/**
 * @param {string} word1
 * @param {string} word2
 * @return {number}
 */
var minDistance = function(word1, word2) {
  let rows = word1.length+1;
  let cols = word2.length+1;
  let dp = new Array(rows).fill(null).map(() => {
    return new Array(cols).fill(0);
  });
  for (let row = 1; row < rows; row += 1) {
    dp[row][0] = row;
  }
  for (let col = 1; col < cols; col += 1) {
    dp[0][col] = col;
  }
  for (let row = 1; row < rows; row += 1) {
    for (let col = 1; col < cols; col += 1) {
      if (word1[row-1] == word2[col-1]) {
        dp[row][col] = 1 + Math.min(dp[row-1][col],
          dp[row][col-1], dp[row-1][col-1]-1);
      } else {
        dp[row][col] = 1 + Math.min(dp[row-1][col],
          dp[row][col-1], dp[row-1][col-1]);
      }
    }
  }
  return dp[word1.length][word2.length];
};
// @lc code=end

