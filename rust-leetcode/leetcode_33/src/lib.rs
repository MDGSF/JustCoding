/*
设置左边界left为0，右边界right为nums.len()-1，取mid=(left+right)/2。
当nums[left]≤nums[mid]时，数组前半部分有序。当nums[left]≤target<nums[mid]时，在数组前半部分找，否则在后半部分找。
当nums[left]>nums[mid]时，数组后半部分有序。当nums[mid]<target≤nums[right]时，则在数组后半部分找，否则在前半部分找。
*/

pub mod solution1;
