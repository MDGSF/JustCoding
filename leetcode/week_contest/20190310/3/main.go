package main

import "fmt"

func test1() {
	A := []int{2, 1, 2, 4, 2, 2}
	B := []int{5, 2, 6, 2, 3, 2}
	fmt.Println("test1 =", minDominoRotations(A, B))
}

func test2() {
	A := []int{3, 5, 1, 2, 3}
	B := []int{3, 6, 3, 3, 4}
	fmt.Println("test2 =", minDominoRotations(A, B))
}

func main() {
	test1()
	test2()
}

func minDominoRotations(A []int, B []int) int {
	type TObj struct {
		AllCount int
		ACount   int
		BCount   int
	}

	m := make(map[int]*TObj)
	for _, v := range A {
		if _, ok := m[v]; ok {
			m[v].AllCount++
			m[v].ACount++
		} else {
			m[v] = &TObj{AllCount: 1, ACount: 1}
		}
	}

	for _, v := range B {
		if _, ok := m[v]; ok {
			m[v].AllCount++
			m[v].BCount++
		} else {
			m[v] = &TObj{AllCount: 1, BCount: 1}
		}
	}

	possible := make([]int, 0)
	for k, v := range m {
		if v.AllCount >= len(A) {
			possible = append(possible, k)
		}
	}

	if len(possible) == 0 {
		return -1
	}

	for _, v := range possible {
		success := true
		for i := 0; i < len(A); i++ {
			if A[i] != v && B[i] != v {
				success = false
				break
			}
		}
		if success {
			obj := m[v]
			if obj.ACount < obj.BCount {
				result := 0
				for i := 0; i < len(A); i++ {
					if A[i] == v && B[i] != v {
						result++
					}
				}
				return result
			} else {
				result := 0
				for i := 0; i < len(A); i++ {
					if A[i] != v && B[i] == v {
						result++
					}
				}
				return result
			}
		}
	}

	return -1
}
