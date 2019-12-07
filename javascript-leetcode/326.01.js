/**
 * @param {number} n
 * @return {boolean}
 */
var isPowerOfThree = function(n) {
  return n > 0 && 1162261467 % n === 0;
};

/*
3^0 = 1
3^1 = 3
3^2 = 9
3^3 = 27
3^4 = 81
...
3^n

更大 3^n 对更小的数字取模一定为零。
比如
9%3 == 0
27%9 == 0
那我们就只要找到最大的那个 3^n 就可以了。

把这题题目切换到 c++，你就会发现参数是 int
而 int 是 32 位，即 2^32
去掉负数之后，就是 2^31 = 2147483648
正数的范围就是 0 ~ 2^32-1，即 0 ~ 2147483647

我们另 3^n = 2147483647
n = log3(2147483647)
再对 n 向下取整。得到 n = 19
然后 3^n 就可以认为是最大的值。

function getBaseLog(x, y) {
  return Math.log(y) / Math.log(x);
}
// 19.55882236029132
console.log(getBaseLog(3, 2147483647));

Math.pow(3, 19) = 1162261467
*/
