package main

import "fmt"

func reverse(passwordBytes []byte) {
	start := 0
	end := len(passwordBytes) - 1
	for start < end {
		passwordBytes[start], passwordBytes[end] = passwordBytes[end], passwordBytes[start]
		start += 1
		end -= 1
	}
}

func rotate(password string, target int) string {
	passwordBytes := []byte(password)
	n := len(passwordBytes)
	target = target % n
	reverse(passwordBytes[:n-target])
	reverse(passwordBytes[n-target : n])
	reverse(passwordBytes[:])
	return string(passwordBytes)
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
