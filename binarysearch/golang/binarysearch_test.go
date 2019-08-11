package binarysearch

import "testing"

func TestBinarySearch1(t *testing.T) {
	a := []int{1, 2, 3, 4, 5, 6, 7, 8}
	ret := BinarySearch(a, 8)
	if ret != 7 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearch2(t *testing.T) {
	a := []int{}
	ret := BinarySearch(a, 8)
	if ret != -1 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearch3(t *testing.T) {
	a := []int{8}
	ret := BinarySearch(a, 8)
	if ret != 0 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearch4(t *testing.T) {
	a := []int{4, 5, 6, 7, 8, 100, 1000, 2000}
	ret := BinarySearch(a, 6)
	if ret != 2 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearch5(t *testing.T) {
	a := []int{4, 5, 6, 7, 8, 100, 1000, 2000}
	ret := BinarySearch(a, 10)
	if ret != -1 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchRecursive1(t *testing.T) {
	a := []int{1, 2, 3, 4, 5, 6, 7, 8}
	ret := BinarySearchRecursive(a, 8)
	if ret != 7 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchRecursive2(t *testing.T) {
	a := []int{}
	ret := BinarySearchRecursive(a, 8)
	if ret != -1 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchRecursive3(t *testing.T) {
	a := []int{8}
	ret := BinarySearchRecursive(a, 8)
	if ret != 0 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchRecursive4(t *testing.T) {
	a := []int{4, 5, 6, 7, 8, 100, 1000, 2000}
	ret := BinarySearchRecursive(a, 6)
	if ret != 2 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchRecursive5(t *testing.T) {
	a := []int{4, 5, 6, 7, 8, 100, 1000, 2000}
	ret := BinarySearchRecursive(a, 10)
	if ret != -1 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchFirst1(t *testing.T) {
	a := []int{1, 2, 3, 4, 5, 6, 7, 8}
	ret := BinarySearchFirst(a, 8)
	if ret != 7 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchFirst2(t *testing.T) {
	a := []int{}
	ret := BinarySearchFirst(a, 8)
	if ret != -1 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchFirst3(t *testing.T) {
	a := []int{8}
	ret := BinarySearchFirst(a, 8)
	if ret != 0 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchFirst4(t *testing.T) {
	a := []int{4, 5, 6, 7, 8, 100, 1000, 2000}
	ret := BinarySearchFirst(a, 6)
	if ret != 2 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchFirst5(t *testing.T) {
	a := []int{4, 5, 6, 7, 8, 100, 1000, 2000}
	ret := BinarySearchFirst(a, 10)
	if ret != -1 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchFirst6(t *testing.T) {
	a := []int{4, 4, 4, 5, 6, 7, 8, 100, 1000, 2000}
	ret := BinarySearchFirst(a, 4)
	if ret != 0 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchFirst7(t *testing.T) {
	a := []int{4, 4, 4, 5, 6, 7, 8, 8, 8, 8, 8, 8, 100, 1000, 2000}
	ret := BinarySearchFirst(a, 8)
	if ret != 6 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchLast1(t *testing.T) {
	a := []int{1, 2, 3, 4, 5, 6, 7, 8}
	ret := BinarySearchLast(a, 8)
	if ret != 7 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchLast2(t *testing.T) {
	a := []int{}
	ret := BinarySearchLast(a, 8)
	if ret != -1 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchLast3(t *testing.T) {
	a := []int{8}
	ret := BinarySearchLast(a, 8)
	if ret != 0 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchLast4(t *testing.T) {
	a := []int{4, 5, 6, 7, 8, 100, 1000, 2000}
	ret := BinarySearchLast(a, 6)
	if ret != 2 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchLast5(t *testing.T) {
	a := []int{4, 5, 6, 7, 8, 100, 1000, 2000}
	ret := BinarySearchLast(a, 10)
	if ret != -1 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchLast6(t *testing.T) {
	a := []int{4, 4, 4, 5, 6, 7, 8, 100, 1000, 2000}
	ret := BinarySearchLast(a, 4)
	if ret != 2 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchLast7(t *testing.T) {
	a := []int{4, 4, 4, 5, 6, 7, 8, 8, 8, 8, 8, 8, 100, 1000, 2000}
	ret := BinarySearchLast(a, 8)
	if ret != 11 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchFirstGT1(t *testing.T) {
	a := []int{1, 2, 3, 4, 5, 6, 7, 8}
	ret := BinarySearchFirstGT(a, 8)
	if ret != -1 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchFirstGT2(t *testing.T) {
	a := []int{}
	ret := BinarySearchFirstGT(a, 8)
	if ret != -1 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchFirstGT3(t *testing.T) {
	a := []int{8}
	ret := BinarySearchFirstGT(a, 8)
	if ret != -1 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchFirstGT4(t *testing.T) {
	a := []int{4, 5, 6, 7, 8, 100, 1000, 2000}
	ret := BinarySearchFirstGT(a, 6)
	if ret != 3 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchFirstGT5(t *testing.T) {
	a := []int{4, 5, 6, 7, 8, 100, 1000, 2000}
	ret := BinarySearchFirstGT(a, 10)
	if ret != 5 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchFirstGT6(t *testing.T) {
	a := []int{4, 4, 4, 5, 6, 7, 8, 100, 1000, 2000}
	ret := BinarySearchFirstGT(a, 4)
	if ret != 3 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchFirstGT7(t *testing.T) {
	a := []int{4, 4, 4, 5, 6, 7, 8, 8, 8, 8, 8, 8, 100, 1000, 2000}
	ret := BinarySearchFirstGT(a, 8)
	if ret != 12 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchFirstGE1(t *testing.T) {
	a := []int{1, 2, 3, 4, 5, 6, 7, 8}
	ret := BinarySearchFirstGE(a, 8)
	if ret != 7 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchFirstGE2(t *testing.T) {
	a := []int{}
	ret := BinarySearchFirstGE(a, 8)
	if ret != -1 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchFirstGE3(t *testing.T) {
	a := []int{8}
	ret := BinarySearchFirstGE(a, 8)
	if ret != 0 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchFirstGE4(t *testing.T) {
	a := []int{4, 5, 6, 7, 8, 100, 1000, 2000}
	ret := BinarySearchFirstGE(a, 6)
	if ret != 2 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchFirstGE5(t *testing.T) {
	a := []int{4, 5, 6, 7, 8, 100, 1000, 2000}
	ret := BinarySearchFirstGE(a, 10)
	if ret != 5 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchFirstGE6(t *testing.T) {
	a := []int{4, 4, 4, 5, 6, 7, 8, 100, 1000, 2000}
	ret := BinarySearchFirstGE(a, 4)
	if ret != 0 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchFirstGE7(t *testing.T) {
	a := []int{4, 4, 4, 5, 6, 7, 8, 8, 8, 8, 8, 8, 100, 1000, 2000}
	ret := BinarySearchFirstGE(a, 8)
	if ret != 6 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchLastLT1(t *testing.T) {
	a := []int{1, 2, 3, 4, 5, 6, 7, 8}
	ret := BinarySearchLastLT(a, 8)
	if ret != 6 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchLastLT2(t *testing.T) {
	a := []int{}
	ret := BinarySearchLastLT(a, 8)
	if ret != -1 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchLastLT3(t *testing.T) {
	a := []int{8}
	ret := BinarySearchLastLT(a, 8)
	if ret != -1 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchLastLT4(t *testing.T) {
	a := []int{4, 5, 6, 7, 8, 100, 1000, 2000}
	ret := BinarySearchLastLT(a, 6)
	if ret != 1 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchLastLT5(t *testing.T) {
	a := []int{4, 5, 6, 7, 8, 100, 1000, 2000}
	ret := BinarySearchLastLT(a, 10)
	if ret != 4 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchLastLT6(t *testing.T) {
	a := []int{4, 4, 4, 5, 6, 7, 8, 100, 1000, 2000}
	ret := BinarySearchLastLT(a, 4)
	if ret != -1 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchLastLT7(t *testing.T) {
	a := []int{4, 4, 4, 5, 6, 7, 8, 8, 8, 8, 8, 8, 100, 1000, 2000}
	ret := BinarySearchLastLT(a, 8)
	if ret != 5 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchLastLE1(t *testing.T) {
	a := []int{1, 2, 3, 4, 5, 6, 7, 8}
	ret := BinarySearchLastLE(a, 8)
	if ret != 7 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchLastLE2(t *testing.T) {
	a := []int{}
	ret := BinarySearchLastLE(a, 8)
	if ret != -1 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchLastLE3(t *testing.T) {
	a := []int{8}
	ret := BinarySearchLastLE(a, 8)
	if ret != 0 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchLastLE4(t *testing.T) {
	a := []int{4, 5, 6, 7, 8, 100, 1000, 2000}
	ret := BinarySearchLastLE(a, 6)
	if ret != 2 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchLastLE5(t *testing.T) {
	a := []int{4, 5, 6, 7, 8, 100, 1000, 2000}
	ret := BinarySearchLastLE(a, 10)
	if ret != 4 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchLastLE6(t *testing.T) {
	a := []int{4, 4, 4, 5, 6, 7, 8, 100, 1000, 2000}
	ret := BinarySearchLastLE(a, 4)
	if ret != 2 {
		t.Fatalf("ret = %v\n", ret)
	}
}

func TestBinarySearchLastLE7(t *testing.T) {
	a := []int{4, 4, 4, 5, 6, 7, 8, 8, 8, 8, 8, 8, 100, 1000, 2000}
	ret := BinarySearchLastLE(a, 8)
	if ret != 11 {
		t.Fatalf("ret = %v\n", ret)
	}
}
