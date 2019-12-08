/**
 * @param {number} n
 * @return {number}
 */
var countPrimes = function(n) {
  let isPrime = Array(n).fill(true);
  isPrime[0] = false;
  isPrime[1] = false;
  for (let i = 2; i < n; i += 1) {
    if (!isPrime[i]) {
      continue;
    }
    for (let j = i + i; j < n; j += i) {
      isPrime[j] = false;
    }
  }
  return isPrime.reduce((acc, cur) => acc + (cur ? 1 : 0), 0);
};

const result = countPrimes(10);
console.log(result);
