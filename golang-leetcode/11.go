package main

import "fmt"

func maxArea(height []int) int {
	result := 0
	left := 0
	right := len(height) - 1
	for left < right {
		curArea := 0
		if height[left] < height[right] {
			curArea = height[left] * (right - left)
			left++
		} else {
			curArea = height[right] * (right - left)
			right--
		}
		if curArea > result {
			result = curArea
		}
	}
	return result
}

func main() {
	fmt.Println("vim-go")
}
