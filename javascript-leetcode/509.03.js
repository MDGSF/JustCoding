/**
 * @param {number} N
 * @return {number}
 */
var fib = function(N) {
  if (N < 2) { return N; }
  let dp = new Array(N+1);
  dp[0] = 0;
  dp[1] = 1;
  for (let i = 2; i <= N; i += 1) {
    dp[i] = dp[i-1] + dp[i-2];
  }
  return dp[N];
};
