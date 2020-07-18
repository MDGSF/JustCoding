package main

import "testing"

func testLengthOfLongestSubstring(t *testing.T, input string, expectRet int) {
	ret := lengthOfLongestSubstring(input)
	if ret != expectRet {
		t.Fatalf("input = %v, ret = %v, expectRet = %v", input, ret, expectRet)
	}
}

func TestLengthOfLongestSubstring1(t *testing.T) {
	testLengthOfLongestSubstring(t, "abcabcbb", 3)
}

func TestLengthOfLongestSubstring2(t *testing.T) {
	testLengthOfLongestSubstring(t, "bbbbb", 1)
}

func TestLengthOfLongestSubstring3(t *testing.T) {
	testLengthOfLongestSubstring(t, "pwwkew", 3)
}

func TestLengthOfLongestSubstring4(t *testing.T) {
	testLengthOfLongestSubstring(t, "", 0)
}

func TestLengthOfLongestSubstring5(t *testing.T) {
	testLengthOfLongestSubstring(t, "a", 1)
}

func TestLengthOfLongestSubstring6(t *testing.T) {
	testLengthOfLongestSubstring(t, "aaabc", 3)
}

func TestLengthOfLongestSubstring7(t *testing.T) {
	testLengthOfLongestSubstring(t, "aaabcabcde", 5)
}

func TestLengthOfLongestSubstring8(t *testing.T) {
	testLengthOfLongestSubstring(t, " ", 1)
}

func TestLengthOfLongestSubstring9(t *testing.T) {
	testLengthOfLongestSubstring(t, "abcdecxyzvn", 8)
}
