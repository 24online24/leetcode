package main

import "fmt"

func trap(height []int) int {
	i, water := 0, 0

	for i < len(height) && height[i] == 0 {
		i++
	}

	for i < len(height)-1 {
		j := i + 1
		side := height[i]
		currentWater := 0
		maximumSide := height[j]
		maximumSideIdx := j
		for ; j < len(height) && height[j] < side; j++ {
			currentWater += side - height[j]
			if height[j] > maximumSide {
				maximumSide, maximumSideIdx = height[j], j
			}
		}
		if j == len(height) {
			for k := i + 1; k < maximumSideIdx; k++ {
				water += maximumSide - height[k]
			}
			i = maximumSideIdx
		} else {
			water += currentWater
			i = j
		}
	}

	return water
}

func main() {
	fmt.Println(trap([]int{0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1})) // 6
	fmt.Println(trap([]int{4, 2, 0, 3, 2, 5}))                   // 9
	fmt.Println(trap([]int{4, 2, 3}))                            // 1
	fmt.Println(trap([]int{5, 0, 1, 0, 0, 0, 0, 0, 0}))
}
