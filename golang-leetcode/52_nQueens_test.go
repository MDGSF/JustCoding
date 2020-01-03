package main

import "testing"

func testTotalNQueens(t *testing.T, n int, expect int) {
	ret := totalNQueens(n)
	if ret != expect {
		t.Fatalf("n = %v, ret = %v, expect = %v", n, ret, expect)
	}
}

func TestTotalNQueens(t *testing.T) {
	testTotalNQueens(t, 4, 2)

}
