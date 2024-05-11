package main

import "slices"

func maximumHappinessSum(happiness []int, k int) int64 {
	slices.SortFunc(happiness, func(a, b int) int { return b - a })
	maximum := 0
	for i := range k {
		hap := happiness[i] - i
		if hap > 0 {
			maximum += hap
		}
	}
	return int64(maximum)
}

func main() {

}
