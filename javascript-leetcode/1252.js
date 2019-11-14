/**
 * @param {number} n
 * @param {number} m
 * @param {number[][]} indices
 * @return {number}
 */
var oddCells = function(n, m, indices) {
  let row = Array(n).fill(0);
  let col = Array(m).fill(0);
  for (let i = 0; i < indices.length; i += 1) {
    row[indices[i][0]] += 1;
    col[indices[i][1]] += 1;
  }
  let count = 0;
  for (let i = 0; i < n; i += 1) {
    for (let j = 0; j < m; j += 1) {
      let sum = row[i] + col[j];
      if (sum % 2 !== 0) {
        count += 1;
      }
    }
  }
  return count;
};
