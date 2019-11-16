/**
 * @param {number[]} bits
 * @return {boolean}
 */
var isOneBitCharacter = function(bits) {
  let i = bits.length - 2;
  let preOneBit = 0;
  while (i >= 0 && bits[i] > 0) {
    preOneBit ^= bits[i];
    i -= 1;
  }
  return preOneBit === 0;
};

/*
从后往前遍历，看最后一个 0 前面一共有多少个 1。
如果 1 的个数是偶数，则最后一个 0 就是第一种字符。
如果 1 的个数是奇数，则最后一个 0 就是第二种字符。
*/
