/**
 * @param {number} N
 * @return {number}
 */
var fib = function(N) {
  return Math.round((Math.pow((1 + Math.sqrt(5))/2 , N) -
      Math.pow((1 - Math.sqrt(5))/2, N)) / Math.sqrt(5));
};

console.log(fib(10));

