package main

func twoSum(nums []int, target int) []int {
	for index, num := range nums {
		for j := index + 1; j < len(nums); j++ {
			if num+nums[j] == target {
				return []int{index, j}
			}
		}
	}
	return []int{}
}

func main() {
}
