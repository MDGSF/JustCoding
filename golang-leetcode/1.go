package main

import "fmt"

func main() {
	fmt.Println("vim-go")
	nums := []int{2, 7, 11, 15}
	target := 9
	result := twoSum(nums, target)
	fmt.Println(result)
}

func twoSum(nums []int, target int) []int {
	m := make(map[int]int)
	for i := 0; i < len(nums); i++ {
		num := nums[i]
		peer := target - num
		if peerIdx, ok := m[peer]; ok {
			return []int{peerIdx, i}
		}
		m[num] = i
	}
	return nil
}

func twoSum_02(nums []int, target int) []int {
	m := make(map[int]int)
	for i := 0; i < len(nums); i++ {
		m[nums[i]] = i
	}
	for i := 0; i < len(nums); i++ {
		num := nums[i]
		peer := target - num
		peerIdx, ok := m[peer]
		if ok && peerIdx != i {
			return []int{i, peerIdx}
		}
	}
	return nil
}

func twoSum_01(nums []int, target int) []int {
	for i := 0; i < len(nums); i++ {
		for j := i + 1; j < len(nums); j++ {
			if nums[i]+nums[j] == target {
				return []int{i, j}
			}
		}
	}
	return nil
}
