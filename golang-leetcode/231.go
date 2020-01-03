package main

import "fmt"

func main() {
	fmt.Println(isPowerOfTwo(2))
}

func isPowerOfTwo(n int) bool {
	count := 0
	for n > 0 {
		if n&0x01 == 0x01 {
			count += 1
		}
		n >>= 1
	}
	return count == 1
}
