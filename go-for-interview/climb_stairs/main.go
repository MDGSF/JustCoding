package main

import "fmt"

/*
n = 1，1阶台阶，1步就直接上去了。
n = 2，2阶台阶。有2种方法可以爬到楼顶。
    1.  1 阶 + 1 阶
    2.  2 阶
n = 3，3阶台阶。 有3种方法可以爬到楼顶。
    1.  1 阶 + 1 阶 + 1 阶
    2.  1 阶 + 2 阶
    3.  2 阶 + 1 阶
*/
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

func main() {
	fmt.Printf("climbStairs(1) = %v\n", climbStairs(1))
	fmt.Printf("climbStairs(2) = %v\n", climbStairs(2))
	fmt.Printf("climbStairs(3) = %v\n", climbStairs(3))
}
