package main

import (
	"fmt"
	"sort"
)

func test1() {
	A := []int{4, 2, 3}
	K := 1
	fmt.Println("vim-go", largestSumAfterKNegations(A, K))
}

func test2() {
	A := []int{3, -1, 0, 2}
	K := 3
	fmt.Println("test2 = ", largestSumAfterKNegations(A, K))
}

func test3() {
	A := []int{2, -3, -1, 5, -4}
	K := 2
	fmt.Println("test3 = ", largestSumAfterKNegations(A, K))
}

func main() {
	test3()
}

func largestSumAfterKNegations(A []int, K int) int {
	sort.Ints(A)

	inActiveNum := 0
	for _, v := range A {
		if v < 0 {
			inActiveNum++
		} else {
			break
		}
	}

	reverseInActiveNum := 0
	leftK := 0
	if K <= inActiveNum {
		reverseInActiveNum = K
	} else {
		reverseInActiveNum = inActiveNum
		leftK = K - reverseInActiveNum
	}

	for i := 0; i < reverseInActiveNum; i++ {
		A[i] = -A[i]
	}

	if leftK > 0 {
		minNum := A[0]
		minIdx := 0
		for i := 1; i < len(A); i++ {
			if A[i] < minNum {
				minNum = A[i]
				minIdx = i
			}
		}
		if leftK%2 != 0 {
			A[minIdx] = -A[minIdx]
		}
	}

	sum := 0
	for _, v := range A {
		sum += v
	}

	return sum
}
