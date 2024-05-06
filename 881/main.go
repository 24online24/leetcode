package main

import (
	"fmt"
	"slices"
)

func numRescueBoats(people []int, limit int) int {
	slices.Sort(people)
	boats := 0
	for i, j := 0, len(people)-1; i <= j; j, boats = j-1, boats+1 {
		if people[i]+people[j] <= limit {
			i++
		}
	}
	return boats
}

func main() {
	fmt.Println(numRescueBoats([]int{1, 2}, 3))
	fmt.Println(numRescueBoats([]int{3, 2, 2, 1}, 3))
	fmt.Println(numRescueBoats([]int{3, 5, 3, 4}, 5))
}
