package main

import (
	"fmt"
	"slices"
	"strconv"
)

func findRelativeRanks(score []int) []string {
	sorted := make([]int, len(score))
	_ = copy(sorted, score)
	slices.Sort(sorted)
	answer := make([]string, len(score))
	for i, el := range score {
		perf := slices.Index(sorted, el)
		var rank string
		switch perf {
		case len(score) - 1:
			rank = "Gold Medal"
		case len(score) - 2:
			rank = "Silver Medal"
		case len(score) - 3:
			rank = "Bronze Medal"
		default:
			rank = strconv.Itoa(len(score) - perf)
		}
		answer[i] = rank
	}
	return answer
}

func main() {
	fmt.Println(findRelativeRanks([]int{5, 4, 3, 2, 1}))
	fmt.Println(findRelativeRanks([]int{10, 3, 8, 9, 4}))
}
