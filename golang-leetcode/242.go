package main

import "fmt"

func main() {
	fmt.Println("vim-go")
	test1()
}

func test1() {
	result := isAnagram("anagram", "nagaram")
	fmt.Println(result)
}

func isAnagram(s string, t string) bool {
	if len(s) != len(t) {
		return false
	}
	m := make([]int, 26)
	for i := range s {
		m[s[i]-97]++
		m[t[i]-97]--
	}
	for i := 0; i < len(m); i++ {
		if m[i] > 0 {
			return false
		}
	}
	return true
}
