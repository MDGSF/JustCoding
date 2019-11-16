/**
 * @param {number[]} bits
 * @return {boolean}
 */
var isOneBitCharacter = function(bits) {
  for (let i = 0; i < bits.length; i += 1) {
    const c = bits[i];
    if (i === bits.length - 1 && c === 0) {
      return true;
    }
    if (c === 1) {
      i += 1;
    }
  }
  return false;
};
