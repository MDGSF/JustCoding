package main

import "fmt"

func maxArea(height []int) int {
	result := 0
	for i := 0; i < len(height)-1; i++ {
		for j := i + 1; j < len(height); j++ {
			curArea := 0
			if height[i] < height[j] {
				curArea = height[i] * (j - i)
			} else {
				curArea = height[j] * (j - i)
			}
			if curArea > result {
				result = curArea
			}
		}
	}
	return result
}

func main() {
	fmt.Println("vim-go")
}
