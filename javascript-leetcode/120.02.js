/**
 * @param {number[][]} triangle
 * @return {number}
 */
var minimumTotal = function(triangle) {
  let dp = new Array(triangle.length + 1).fill(0);
  for (let row = triangle.length - 1; row >= 0; row -= 1) {
    for (let col = 0; col < triangle[row].length; col += 1) {
      dp[col] = Math.min(dp[col], dp[col+1]) + triangle[row][col];
    }
  }
  return dp[0];
};
