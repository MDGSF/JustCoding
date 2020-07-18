package main

import (
	"fmt"
	"strings"
)

func main() {
	fmt.Println("vim-go")
}

func lengthOfLongestSubstring(s string) int {
	return lengthOfLongestSubstring3(s)
}

func lengthOfLongestSubstring3(s string) int {
	ans, i := 0, 0
	for j := 0; j < len(s); j++ {
		t := strings.Index(s[i:j], string(s[j]))
		if t == -1 {
			if ans < j-i+1 {
				ans = j - i + 1
			}
		} else {
			i = i + t + 1
		}
	}
	return ans
}

/*
lengthOfLongestSubstring2

int [26] 用于字母 ‘a’ - ‘z’ 或 ‘A’ - ‘Z’
int [128] 用于ASCII码
int [256] 用于扩展ASCII码

sliding window: [i, j)

m 是记录字符到下标的映射
*/
func lengthOfLongestSubstring2(s string) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}

	ans := 0
	m := make(map[byte]int)
	for i, j := 0, 0; j < len(s); j++ {
		if idx, ok := m[s[j]]; ok {
			i = max(i, idx+1)
		}
		ans = max(ans, j-i+1)
		m[s[j]] = j
	}
	return ans
}

/*
lengthOfLongestSubstring1
exists 是一个字符集合，记录字符在窗口 (i, j) 之间的字符是否存在。
*/
func lengthOfLongestSubstring1(s string) int {
	if len(s) <= 1 {
		return len(s)
	}

	i, j := -1, 0
	exists := make(map[byte]struct{})
	maxCount := 0
	curCount := 0

	for j < len(s) {
		ch := s[j]
		if _, ok := exists[ch]; ok {
			if curCount > maxCount {
				maxCount = curCount
			}
			i++
			delete(exists, s[i])
			curCount--
		} else {
			exists[ch] = struct{}{}
			curCount++
			j++
		}
	}

	if curCount > maxCount {
		maxCount = curCount
	}

	return maxCount
}
