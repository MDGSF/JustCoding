package main

import (
	"fmt"
	"strings"
)

func main() {
	fmt.Println("vim-go")
}

func convert(s string, numRows int) string {
	return convert4(s, numRows)
}

func convert4(s string, numRows int) string {
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}

	if numRows == 1 {
		return s
	}

	results := make([]string, min(numRows, len(s)))
	goingDown := false
	idx := 0

	for _, ch := range s {
		results[idx] += string(ch)

		if idx == 0 || idx == numRows-1 {
			goingDown = !goingDown
		}

		if goingDown {
			idx++
		} else {
			idx--
		}
	}

	return strings.Join(results, "")
}

func convert3(s string, numRows int) string {
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}

	if numRows == 1 {
		return s
	}

	results := make([]string, min(numRows, len(s)))
	goingDown := true
	idx := 0

	for _, ch := range s {
		results[idx] += string(ch)
		if goingDown {
			if idx == numRows-1 {
				idx--
				goingDown = false
			} else {
				idx++
			}
		} else {
			if idx == 0 {
				idx++
				goingDown = true
			} else {
				idx--
			}
		}
	}

	return strings.Join(results, "")
}

func convert2(s string, numRows int) string {
	if numRows == 1 {
		return s
	}

	ans := ""
	length := len(s)
	cycleLen := numRows + (numRows - 2)
	for i := 0; i < numRows; i++ {
		t := numRows - 1 - i
		for j := i; j < length; j += cycleLen {
			ans += string(s[j])

			j2 := j + 2*t
			if !(i == 0 || i == numRows-1 || j2 >= length) {
				ans += string(s[j2])
			}
		}
	}
	return ans
}

func convert1(s string, numRows int) string {
	if numRows == 1 {
		return s
	}

	ans := ""
	length := len(s)
	end := numRows + (numRows - 2)
	for i := 0; i < numRows; i++ {
		if i == 0 || i == numRows-1 {
			for j := i; j < length; j += end {
				ans += string(s[j])
			}
		} else {
			t := numRows - 1 - i
			for j := i; j < length; j += end {

				ans += string(s[j])

				j2 := j + 2*t
				if j2 >= length {
					break
				}
				ans += string(s[j2])
			}
		}
	}
	return ans
}
