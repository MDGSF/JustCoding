package main

import "fmt"

func reverse(array []int) {
	i := 0
	j := len(array) - 1
	for i < j {
		array[i], array[j] = array[j], array[i]
		i++
		j--
	}
}

func rotateArray(array []int, k int) {
	k = k % len(array)
	reverse(array[0:k])
	reverse(array[k:len(array)])
	reverse(array[0:len(array)])
}

func main() {
	array := []int{1, 2, 3, 4, 5, 6, 7, 8, 9}
	k := 3
	rotateArray(array, k)
	fmt.Println(array) // 4,5,6,7,8,9,1,2,3
}
