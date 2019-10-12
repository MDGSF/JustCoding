/*
@brief calculate hamming distance of two string a and b.
@param {string} a
@param {string} b
@return {number}
*/
function hammingDistance(a, b) {
  if (a.length !== b.length) {
    throw new Error("Strings must be of the same length");
  }

  let distance = 0;

  for (let i = 0; i < a.length; i++) {
    if (a[i] !== b[i]) {
      distance += 1;
    }
  }

  return distance;
}

module.exports = hammingDistance;
