/**
 * @param {number} N
 * @return {number}
 */
var fib = function(N) {
  if (N < 2) { return N; }
  return fib(N-1) + fib(N-2);
};
