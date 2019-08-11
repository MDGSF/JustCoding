package main

import "fmt"

func test1() {
	fmt.Println("test1 = ", clumsy(4))
}

func test2() {
	fmt.Println("test2 = ", clumsy(10))
}

func test3() {
	fmt.Println("test3 = ", clumsy(3))
}

func main() {
	test1()
	test2()
	test3()
}

func clumsy(N int) int {
	result := 0
	first := true
	for N >= 4 {
		if first {
			cur := N*(N-1)/(N-2) + (N - 3)
			result += cur
			first = false
		} else {
			cur := N*(N-1)/(N-2) - (N - 3)
			result -= cur
		}

		N -= 4
	}

	for N > 0 {
		cur := N
		if N-1 > 0 {
			cur = cur * (N - 1)
		}
		if N-2 > 0 {
			cur = cur / (N - 2)
		}
		if N-3 > 0 {
			if first {
				cur = cur + (N - 3)
			} else {
				cur = cur - (N - 3)
			}
		}

		if first {
			result += cur
			first = false
		} else {
			result -= cur
		}
		break
	}

	return result
}
