/**
 * // This is the CustomFunction's API interface.
 * // You should not implement it, or speculate about its implementation
 * function CustomFunction() {
 *
 *     @param {integer, integer} x, y
 *     @return {integer}
 *     this.f = function(x, y) {
 *         ...
 *     };
 *
 * };
 */
/**
 * @param {CustomFunction} customfunction
 * @param {integer} z
 * @return {integer[][]}
 */
var findSolution = function(customfunction, z) {
    const result = [];
    for (let i = 1; i <= 1000; i += 1) {
      for (let j = 1; j <= 1000; j += 1) {
        if (customfunction.f(i, j) === z) {
          result.push([i, j]);
        }
      }
    }
    return result;
  };