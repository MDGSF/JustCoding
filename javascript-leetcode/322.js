/**
 * @param {number[]} coins
 * @param {number} amount
 * @return {number}
 */
/*
F(s) = min(F(s - c1), F(s - c2), F(s - c3), ...) + 1
F(s) 是对于金额 s 最少的硬币数
c1, c2, c3 是硬币的金额数
*/
var coinChange = function(coins, amount) {
  if (amount === 0) {
    return 0;
  }

  const F = new Map();
  for (let i = 0; i < coins.length; i += 1) {
    const coin = coins[i];
    F.set(coin, 1);
  }

  for (let i = 1; i <= amount; i += 1) {
    if (F.has(i)) {
      continue;
    }

    let minChange = coins
      .map(coin => i - coin)
      .filter(coin => coin > 0 && F.has(coin))
      .map(coin => F.get(coin));
    if (minChange.length === 0) {
      continue;
    }

    minChange = minChange.reduce((x, y) => (x < y ? x : y), Number.MAX_VALUE);
    F.set(i, minChange + 1);
  }

  return F.has(amount) ? F.get(amount) : -1;
};

const coins = [1, 2, 5];
const amount = 11;
//const coins = [2];
//const amount = 4;
const result = coinChange(coins, amount);
console.log(result);
