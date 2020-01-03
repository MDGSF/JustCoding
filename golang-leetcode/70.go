package main

import "fmt"

func main() {
	fmt.Println("climbStairs(3) = ", climbStairs(3))
}

func climbStairs(n int) int {
	if n <= 2 {
		return n
	}
	i, j := 1, 2
	for idx := 3; idx <= n; idx++ {
		i, j = j, i+j
	}
	return j
}
