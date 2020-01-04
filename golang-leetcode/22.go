package main

import "fmt"

func main() {
	fmt.Printf("%v\n", generateParenthesis(3))
}

func generateParenthesis(n int) []string {
	return recursion(0, 0, n, "")
}

func recursion(left, right, n int, strs string) []string {
	if left == n && right == n {
		return []string{strs}
	}
	result := make([]string, 0)
	if left < n {
		sub := recursion(left+1, right, n, strs+"(")
		result = append(result, sub...)
	}
	if right < left {
		sub := recursion(left, right+1, n, strs+")")
		result = append(result, sub...)
	}
	return result
}
