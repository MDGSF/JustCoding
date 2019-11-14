/**
 * @param {number[][]} matrix
 * @return {boolean}
 */
var isToeplitzMatrix = function(matrix) {
  if (matrix.length === 0) {
    return true;
  }
  let columnNum = matrix[0].length;
  let preRow = matrix[0];
  for (let i = 1; i < matrix.length; i += 1) {
    let curRow = matrix[i];
    for (let j = 0; j < columnNum - 1; j += 1) {
      if (preRow[j] !== curRow[j + 1]) {
        return false;
      }
    }
    preRow = curRow;
  }
  return true;
};
