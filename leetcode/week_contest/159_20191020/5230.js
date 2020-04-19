/**
 * @param {number[][]} coordinates
 * @return {boolean}
 */
var checkStraightLine = function(coordinates) {
  let p0 = coordinates[0];
  let p1 = coordinates[1];
  const flag = (p1[1] - p0[1]) / (p1[0] - p0[0]);
  for (let i = 3; i < coordinates.length; i += 1) {
    p1 = coordinates[i];
    p0 = coordinates[i - 1];
    const curFlag = (p1[1] - p0[1]) / (p1[0] - p0[0]);
    if (curFlag !== flag) {
      return false;
    }
  }
  return true;
};

// ret = checkStraightLine([[1, 2], [2, 3], [3, 4], [4, 5], [5, 6], [6, 7]]);
// ret = checkStraightLine([[1, 1], [2, 2], [3, 4], [4, 5], [5, 6], [7, 7]]);
console.log(ret);
