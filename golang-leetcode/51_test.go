package main

import "testing"

func testSolveNQueens(t *testing.T, n int, expect [][]string) {
	ret := solveNQueens(n)
	if len(ret) != len(expect) {
		t.Fatalf("n = %v, ret = %v, expect = %v", n, ret, expect)
	}
	for i := 0; i < len(ret); i++ {
		if len(ret[i]) != len(expect[i]) {
			t.Fatalf("n = %v, ret = %v, expect = %v", n, ret, expect)
		}
		for j := 0; j < len(ret[i]); j++ {
			if ret[i][j] != expect[i][j] {
				t.Fatalf("n = %v, ret = %v, expect = %v", n, ret, expect)
			}
		}
	}
}

func TestSolveNQueens(t *testing.T) {
	testSolveNQueens(t, 4, [][]string{
		{
			".Q..",
			"...Q",
			"Q...",
			"..Q.",
		},
		{
			"..Q.",
			"Q...",
			"...Q",
			".Q..",
		},
	})

}
