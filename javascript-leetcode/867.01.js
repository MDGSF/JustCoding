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

  let srcRow = 0;
  let srcCol = 0;

  for (let i = 0; i < A[0].length; i += 1) {
    for (let j = 0; j < A.length; j += 1) {
      newA[i][j] = A[srcRow][srcCol];
      if (srcRow === A.length - 1) {
        srcRow = 0;
        srcCol += 1;
      } else {
        srcRow += 1;
      }
    }
  }

  return newA;
};
