package main

import "testing"

func testMyPow(t *testing.T, x float64, n int, expect float64) {
	ret := myPow(x, n)
	if !(ret-expect < 0.00001 && ret-expect > -0.00001) {
		t.Fatalf("x = %v, n = %v, ret = %v, expect = %v", x, n, ret, expect)
	}
}

func TestMyPow1(t *testing.T) {
	testMyPow(t, 2, 10, 1024)
}

func TestMyPow2(t *testing.T) {
	testMyPow(t, 2.1, 3, 9.261)
}

func TestMyPow3(t *testing.T) {
	testMyPow(t, 2, -2, 0.25)
}

func TestMyPow4(t *testing.T) {
	testMyPow(t, 2, 0, 1)
}

func TestMyPow5(t *testing.T) {
	testMyPow(t, 1, 200, 1)
}
