package main

func findMaxK(nums []int) int {
	present := make(map[int]bool)
	maximum := 0
	for _, num := range nums {
		if present[-num] {
			posNum := num
			if num < 0 {
				posNum = -num
			}
			if posNum > maximum {
				maximum = posNum
			}
		} else {
			present[num] = true
		}
	}
	return maximum
}

func main() {

}
