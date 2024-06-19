package main

import "fmt"

func rotate(s string, k int) string {
	n := len(s)
	k = k % n
	return s[n-k:] + s[:n-k]
}

func test01() {
	ret := rotate("abcdefg", 3)
	if ret != "efgabcd" {
		panic(fmt.Sprintf("ret = %v", ret))
	}
}

func test02() {
	ret := rotate("abcdefg", 13)
	if ret != "bcdefga" {
		panic(fmt.Sprintf("ret = %v", ret))
	}
}

func main() {
	test01()
	test02()
}
