/**
 * @param {number[][]} A
 * @return {number[][]}
 */
var transpose = function(A) {
  let newA = Array(A[0].length)
    .fill(null)
    .map(() => {
      return Array(A.length).fill(null);
    });

  for (let r = 0; r < A.length; r += 1) {
    for (let c = 0; c < A[0].length; c += 1) {
      newA[c][r] = A[r][c];
    }
  }

  return newA;
};
