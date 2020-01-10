/**
 * @param {number} N
 * @return {number}
 */
var fib = function(N) {
  const m = new Map();
  m.set(0, 0);
  m.set(1, 1);
  return recursion(N, m);
};

function recursion(N, m) {
  if (m.has(N)) {
    return m.get(N);
  }
  const result = recursion(N-1, m) + recursion(N-2, m);
  m.set(N, result);
  return result;
}

for (let i = 0; i < 10; i += 1) {
  const result = fib(i);
  console.log(result);
}
