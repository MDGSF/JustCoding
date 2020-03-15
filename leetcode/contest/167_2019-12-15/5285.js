/**
 * @param {number[][]} mat
 * @param {number} threshold
 * @return {number}
 */
var maxSideLength = function(mat, threshold) {
  let matH = mat.length;
  let matW = mat[0].length;
  let squareMaxWidth = Math.min(matH, matW);
  const m = new Map();
  let lowSquareWidth = 1;
  let highSquareWidth = squareMaxWidth;
  let maxResultW = 0;
  while (lowSquareWidth <= highSquareWidth) {
    let hasValid = false;
    let w = Math.floor((lowSquareWidth + highSquareWidth) / 2);
    for (let row = 0; row < matH && row + w <= matH; row += 1) {
      for (let col = 0; col < matW && col + w <= matW; col += 1) {
        let cur = null;

        if (row === 0 && col === 0) {
          cur = calcArea(mat, row, col, w);
        } else if (col === 0) {
          let topKey = getKey(row - 1, col, w);
          let topArea = m.get(topKey);
          cur = topArea;

          // sub top row
          for (let i = col; i < col + w; i += 1) {
            cur = cur - mat[row - 1][i];
          }

          // add bottom row
          for (let i = col; i < col + w; i += 1) {
            cur = cur + mat[row + w - 1][i];
          }

        } else {
          let leftKey = getKey(row, col - 1, w);
          let leftArea = m.get(leftKey)
          cur = leftArea;

          // sub left col
          for (let i = row; i < row + w; i += 1) {
            cur = cur - mat[i][col - 1];
          }

          // add right col
          for (let i = row; i < row + w; i += 1) {
            cur = cur + mat[i][col + w - 1];
          }
        }

        let key = getKey(row, col, w);
        m.set(key, cur);

        if (cur <= threshold) {
          maxResultW = Math.max(maxResultW, w);
          hasValid = true;
        }
      }
    }

    if (hasValid) {
      lowSquareWidth = w + 1;
    } else {
      highSquareWidth = w - 1;
    }
  }
  return maxResultW;
};

function getKey(row, col, w) {
  return `${row}_${col}_${w}`
}

function calcArea(mat, row, col, w) {
  let area = 0;
  for (let i = row; i < row + w; i += 1) {
    for (let j = col; j < col + w; j += 1) {
      area += mat[i][j];
    }
  }
  return area;
}

//const mat = [[1,1,3,2,4,3,2],[1,1,3,2,4,3,2],[1,1,3,2,4,3,2]];
//const threshold = 4;
//2

//const mat = [[2,2,2,2,2],[2,2,2,2,2],[2,2,2,2,2],[2,2,2,2,2],[2,2,2,2,2]];
//const threshold = 1;
//0

//mat = [[1,1,1,1],[1,0,0,0],[1,0,0,0],[1,0,0,0]];
//threshold = 6
// 3

//mat = [[18,70],[61,1],[25,85],[14,40],[11,96],[97,96],[63,45]];
//threshold = 40184
//2

const result = maxSideLength(mat, threshold);
console.log(result);
