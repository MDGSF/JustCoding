package main

import "fmt"

func moveZeroes(nums []int) {
	j := 0
	for i := range nums {
		if nums[i] != 0 {
			nums[i], nums[j] = nums[j], nums[i]
			j++
		}
	}
}

func main() {
	nums := []int{0, 1, 0, 3, 12}
	fmt.Printf("nums = %v\n", nums)

	moveZeroes(nums)
	fmt.Printf("nums = %v\n", nums)
}
