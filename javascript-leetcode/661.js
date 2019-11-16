/**
 * @param {number[][]} M
 * @return {number[][]}
 */
var imageSmoother = function(M) {
  let result = Array(M.length)
    .fill(null)
    .map(() => {
      return Array(M[0].length).fill(null);
    });

  for (let i = 0; i < M.length; i += 1) {
    for (let j = 0; j < M[0].length; j += 1) {
      result[i][j] = calcCellAvg(M, i, j);
    }
  }

  return result;
};

const directions = [
  [-1, -1],
  [-1, 0],
  [-1, 1],
  [0, -1],
  [0, 1],
  [1, -1],
  [1, 0],
  [1, 1],
];

function calcCellAvg(M, row, col) {
  let cellNum = 1;
  let cellSums = M[row][col];
  for (let i = 0; i < directions.length; i += 1) {
    let newRow = row + directions[i][0];
    let newCol = col + directions[i][1];
    if (isValid(M, newRow, newCol)) {
      cellNum += 1;
      cellSums += M[newRow][newCol];
    }
  }
  return Math.floor(cellSums / cellNum);
}

function isValid(M, row, col) {
  if (row < 0 || col < 0) {
    return false;
  }
  if (row >= M.length) {
    return false;
  }
  if (col >= M[0].length) {
    return false;
  }
  return true;
}
