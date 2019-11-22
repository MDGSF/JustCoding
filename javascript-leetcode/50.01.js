/**
 * @param {number} x
 * @param {number} n
 * @return {number}
 */
var myPow = function(x, n) {
  if (n < 0) {
    x = 1 / x;
    n = -n;
  }
  let pow = 1;
  while (n > 0) {
    if ((n & 1) === 1) {
      pow *= x;
    }
    x *= x;
    n >>>= 1;
  }
  return pow;
};

const result = myPow(2.0, -2147483648);
console.log(result);
