package main

import (
	"fmt"
)

func absDiff(x byte, y byte) byte {
	if x < y {
		return y - x
	}
	return x - y
}

func longestIdealString(s string, k int) int {
	idealStringLength := make([]int, len(s))
	longest := 0
	for i := range s {
		idealStringLength[i] = 1
		for j := i - 1; j >= 0; j-- {
			if int(absDiff(s[i], s[j])) <= k && idealStringLength[j]+1 > idealStringLength[i] {
				idealStringLength[i] = idealStringLength[j] + 1
			}
			if idealStringLength[j] == longest {
				break
			}
		}
		if idealStringLength[i] > longest {
			longest = idealStringLength[i]
		}
	}
	return longest
}

func main() {
	fmt.Println(longestIdealString("acfgbd", 2))
	fmt.Println(longestIdealString("abcd", 3))
	fmt.Println(longestIdealString("eduktdb", 15))
}
