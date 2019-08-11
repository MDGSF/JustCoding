package binarysearch

// BinarySearch binary search
// 二分查找，非递归版本
func BinarySearch(a []int, v int) int {
	if len(a) == 0 {
		return -1
	}

	low := 0
	high := len(a) - 1
	for low <= high {
		mid := low + (high-low)/2
		if a[mid] == v {
			return mid
		} else if a[mid] > v {
			high = mid - 1
		} else {
			low = mid + 1
		}
	}
	return -1
}

// BinarySearchRecursive binary search
// 二分查找，递归版本
func BinarySearchRecursive(a []int, v int) int {
	if len(a) == 0 {
		return -1
	}
	return BinarySearchRecursiveInner(a, v, 0, len(a)-1)
}

func BinarySearchRecursiveInner(a []int, v int, low int, high int) int {
	if low > high {
		return -1
	}
	mid := low + (high-low)/2
	if a[mid] == v {
		return mid
	} else if a[mid] > v {
		return BinarySearchRecursiveInner(a, v, low, mid-1)
	} else {
		return BinarySearchRecursiveInner(a, v, mid+1, high)
	}
}

// BinarySearchFirst binary search first element equal to v
// 查找第一个值等于给定值的元素
func BinarySearchFirst(a []int, v int) int {
	if len(a) == 0 {
		return -1
	}

	low := 0
	high := len(a) - 1
	for low <= high {
		mid := low + ((high - low) >> 2)
		if a[mid] > v {
			high = mid - 1
		} else if a[mid] < v {
			low = mid + 1
		} else {
			if mid == 0 || a[mid-1] != v {
				return mid
			} else {
				high = mid - 1
			}
		}
	}
	return -1
}

// BinarySearchLast binary search last element equal to v
// 查找最后一个值等于给定值的元素
func BinarySearchLast(a []int, v int) int {
	n := len(a)
	if n == 0 {
		return -1
	}

	low := 0
	high := n - 1
	for low <= high {
		mid := low + ((high - low) >> 2)
		if a[mid] > v {
			high = mid - 1
		} else if a[mid] < v {
			low = mid + 1
		} else {
			if mid == n-1 || a[mid+1] != v {
				return mid
			} else {
				low = mid + 1
			}
		}
	}
	return -1
}

// BinarySearchFirstGT binary search first element greater than v
// 查找第一个大于给定值的元素
func BinarySearchFirstGT(a []int, v int) int {
	n := len(a)
	if n == 0 {
		return -1
	}

	low := 0
	high := n - 1
	for low <= high {
		mid := low + ((high - low) >> 2)
		if a[mid] > v {
			if (mid == 0) || (a[mid-1] <= v) {
				return mid
			} else {
				high = mid - 1
			}
		} else {
			low = mid + 1
		}

	}
	return -1
}

// BinarySearchFirstGE binary search first element greater or equal to v
// 查找第一个大于等于给定值的元素
func BinarySearchFirstGE(a []int, v int) int {
	n := len(a)
	if n == 0 {
		return -1
	}

	low := 0
	high := n - 1
	for low <= high {
		mid := low + ((high - low) >> 2)
		if a[mid] >= v {
			if (mid == 0) || (a[mid-1] < v) {
				return mid
			} else {
				high = mid - 1
			}
		} else {
			low = mid + 1
		}
	}

	return -1
}

// BinarySearchLastLT binary search last element less than v
// 查找最后一个小于给定值的元素
func BinarySearchLastLT(a []int, v int) int {
	n := len(a)
	if n == 0 {
		return -1
	}

	low := 0
	high := n - 1
	for low <= high {
		mid := low + ((high - low) >> 2)
		if a[mid] >= v {
			high = mid - 1
		} else {
			if (mid == n-1) || (a[mid+1] >= v) {
				return mid
			} else {
				low = mid + 1
			}
		}
	}

	return -1
}

// BinarySearchLastLE binary search last element less or equal to v
// 查找最后一个小于等于给定值的元素
func BinarySearchLastLE(a []int, v int) int {
	n := len(a)
	if n == 0 {
		return -1
	}

	low := 0
	high := n - 1
	for low <= high {
		mid := low + ((high - low) >> 2)
		if a[mid] > v {
			high = mid - 1
		} else {
			if (mid == n-1) || (a[mid+1] > v) {
				return mid
			} else {
				low = mid + 1
			}
		}
	}

	return -1
}
