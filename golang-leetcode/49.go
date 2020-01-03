package main

import (
	"fmt"
	"sort"
	"strings"
)

func main() {
	fmt.Println("vim-go")
	strs := []string{"eat", "tea", "tan", "ate", "nat", "bat"}
	result := groupAnagrams(strs)
	fmt.Println(result)
}

func groupAnagrams(strs []string) [][]string {
	m := make(map[string][]string)
	for _, str := range strs {
		letters := strings.Split(str, "")
		sort.Strings(letters)
		key := strings.Join(letters, "")
		if _, ok := m[key]; ok {
			m[key] = append(m[key], str)
		} else {
			m[key] = []string{str}
		}
	}

	result := make([][]string, 0)
	for key := range m {
		result = append(result, m[key])
	}
	return result
}
