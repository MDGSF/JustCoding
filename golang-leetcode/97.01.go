package main

import "fmt"

func isInterleave(s1 string, s2 string, s3 string) bool {
	n := len(s1)
	m := len(s2)
	t := len(s3)
	if (n + m) != t {
		return false
	}

	f := make([][]bool, n+1)
	for i := 0; i <= n; i++ {
		f[i] = make([]bool, m+1)
	}
	f[0][0] = true
	for i := 0; i <= n; i++ {
		for j := 0; j <= m; j++ {
			p := i + j - 1
			if i > 0 {
				f[i][j] = f[i][j] || (f[i-1][j] && s1[i-1] == s3[p])
			}
			if j > 0 {
				f[i][j] = f[i][j] || (f[i][j-1] && s2[j-1] == s3[p])
			}
		}
	}

	return f[n][m]
}

func main() {
	//s1 := "aabcc"
	//s2 := "dbbca"
	//s3 := "aadbbcbcac"

	s1 := "aabcc"
	s2 := "dbbca"
	s3 := "aadbbbaccc"
	result := isInterleave(s1, s2, s3)
	fmt.Println("result = ", result)
}
