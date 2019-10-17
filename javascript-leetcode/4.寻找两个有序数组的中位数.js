/*
 * @lc app=leetcode.cn id=4 lang=javascript
 *
 * [4] 寻找两个有序数组的中位数
 *
 * https://leetcode-cn.com/problems/median-of-two-sorted-arrays/description/
 *
 * algorithms
 * Hard (36.10%)
 * Likes:    1638
 * Dislikes: 0
 * Total Accepted:    102.6K
 * Total Submissions: 283.8K
 * Testcase Example:  '[1,3]\n[2]'
 *
 * 给定两个大小为 m 和 n 的有序数组 nums1 和 nums2。
 *
 * 请你找出这两个有序数组的中位数，并且要求算法的时间复杂度为 O(log(m + n))。
 *
 * 你可以假设 nums1 和 nums2 不会同时为空。
 *
 * 示例 1:
 *
 * nums1 = [1, 3]
 * nums2 = [2]
 *
 * 则中位数是 2.0
 *
 *
 * 示例 2:
 *
 * nums1 = [1, 2]
 * nums2 = [3, 4]
 *
 * 则中位数是 (2 + 3)/2 = 2.5
 *
 *
 */

// @lc code=start
/**
 * @param {number[]} nums1
 * @param {number[]} nums2
 * @return {number}
 */
var findMedianSortedArrays = function(nums1, nums2) {
  class GetMid {
    constructor(num1, num2) {
      this.num1 = num1;
      this.num2 = num2;
      this.allNums = num1.length + num2.length;
      this.isOdd = null;
      this.isEven = null;
      this.mid = null;
      if (this.allNums % 2 === 0) {
        this.isOdd = false;
        this.isEven = true;
        this.mid = Math.floor(this.allNums / 2) - 1;
      } else {
        this.isOdd = true;
        this.isEven = false;
        this.mid = Math.floor(this.allNums / 2);
      }
      this.idx1 = 0;
      this.idx2 = 0;
    }

    getNextNumber() {
      let n1 = null;
      let n2 = null;
      if (this.idx1 < this.num1.length) {
        n1 = this.num1[this.idx1];
      }
      if (this.idx2 < this.num2.length) {
        n2 = this.num2[this.idx2];
      }
      if (n1 === null) {
        this.idx2 += 1;
        return n2;
      }
      if (n2 === null) {
        this.idx1 += 1;
        return n1;
      }
      if (n1 < n2) {
        this.idx1 += 1;
        return n1;
      }
      this.idx2 += 1;
      return n2;
    }

    moveNext() {
      let n1 = null;
      let n2 = null;
      if (this.idx1 < this.num1.length) {
        n1 = this.num1[this.idx1];
      }
      if (this.idx2 < this.num2.length) {
        n2 = this.num2[this.idx2];
      }
      if (n1 === null) {
        this.idx2 += 1;
        return;
      }
      if (n2 === null) {
        this.idx1 += 1;
        return;
      }

      if (n1 < n2) {
        this.idx1 += 1;
        return;
      }
      this.idx2 += 1;
    }

    calculate() {
      while (this.idx1 < this.num1.length || this.idx2 < this.num2.length) {
        if (this.idx1 + this.idx2 === this.mid) {
          if (this.isOdd) {
            return this.getNextNumber();
          }
          const n1 = this.getNextNumber();
          const n2 = this.getNextNumber();
          return (n1 + n2) / 2;
        }
        this.moveNext();
      }
    }
  }

  const t = new GetMid(nums1, nums2);
  return t.calculate();
};
// @lc code=end
