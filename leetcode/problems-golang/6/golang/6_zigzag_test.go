package main

import "testing"

func testConvert(t *testing.T, s string, numRows int, expect string) {
	ans := convert(s, numRows)
	if ans != expect {
		t.Fatalf("s = %v, numRows = %v, ans = %v, expect = %v", s, numRows, ans, expect)
	}
}

func TestConvert1(t *testing.T) {
	testConvert(t, "LEETCODEISHIRING", 3, "LCIRETOESIIGEDHN")
}

func TestConvert2(t *testing.T) {
	testConvert(t, "LEETCODEISHIRING", 4, "LDREOEIIECIHNTSG")
}

func TestConvert3(t *testing.T) {
	testConvert(t, "LEETCODEISHIRING", 1, "LEETCODEISHIRING")
}
