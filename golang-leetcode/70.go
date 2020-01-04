package main

import "fmt"

func main() {
	fmt.Println("climbStairs(3) = ", climbStairs(3))
}

func climbStairs(n int) int {
	if n < 3 {
		return n
	}
	f1, f2 := 1, 2
	for i := 3; i <= n; i++ {
		f1, f2 = f2, f1+f2
	}
	return f2
}
