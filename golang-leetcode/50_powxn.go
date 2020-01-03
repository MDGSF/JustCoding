package main

import "fmt"

func main() {
	fmt.Println("vim-go")
}

/*
n 为偶数： x^n = x^(n/2) * x^(n/2)
n 为奇数： x^n = x^(n/2) * x^(n/2) * x

x^(-n) = 1/(x^n)
*/

func myPow(x float64, n int) float64 {
	return myPow2(x, n)
}

/*
举例：
n = 10, 二进制为 1010
那么 x^n = x^10 = x^8 * x^2，这里的 8 和 2 就对应 10 的二进制上的 1 的位置。

二进制 1010 的每一位对应的 current 为：
current^8, current^4, current^2, current^1
*/
func myPow2(x float64, n int) float64 {
	if n == 0 {
		return 1
	}

	if n < 0 {
		x = 1 / x
		n = -n
	}

	var ans float64 = 1
	current := x
	for n > 0 {
		if n&1 == 1 {
			ans *= current
		}
		current = current * current
		n >>= 1
	}
	return ans
}

func myPow1(x float64, n int) float64 {
	if n == 0 {
		return 1
	}

	if n < 0 {
		return 1 / myPow(x, -n)
	}

	ret := myPow(x, n/2)
	if n%2 == 0 {
		return ret * ret
	}
	return ret * ret * x
}
