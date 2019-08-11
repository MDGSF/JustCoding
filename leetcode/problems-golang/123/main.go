package main

import "fmt"

func main() {
	arr := []int{1, 2, 3, 4, 5}
	fmt.Println("vim-go", maxProfit(arr))
}

func maxProfit(prices []int) int {

	fmt.Println(prices)

	n := len(prices)
	const K int = 2

	if n == 0 {
		return 0
	}

	DP := make([][K + 1][2]int, n)

	maxint := int(^uint32(0) >> 1)
	minint := ^maxint

	DP[0][0][0] = 0          //第 0 天，什么也没做
	DP[0][0][1] = -prices[0] //第 0 天，买入股票
	DP[0][1][0] = minint     //no
	DP[0][1][1] = minint     //no
	DP[0][2][0] = minint     //no
	DP[0][2][1] = minint     //no

	for i := 1; i < n; i++ {
		for k := 0; k <= K; k++ {
			if k == 0 {
				DP[i][0][0] = DP[i-1][0][0]
			} else {
				DP[i][k][0] = max(DP[i-1][k][0], DP[i-1][k-1][1]+prices[i])
			}

			DP[i][k][1] = max(DP[i-1][k][1], DP[i-1][k][0]-prices[i])
		}
	}

	for i := 0; i < n; i++ {
		for k := 0; k <= K; k++ {
			for j := 0; j < 2; j++ {
				fmt.Printf("[%v][%v][%v] = %v\n", i, k, j, DP[i][k][j])
			}
		}
	}

	m := DP[n-1][0][0]
	for k := 1; k <= K; k++ {
		if DP[n-1][k][0] > m {
			m = DP[n-1][k][0]
		}
	}
	return m
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}
