package main

import "fmt"

func generate(numRows int) [][]int {
	var triangle [][]int
	triangle = append(triangle, []int{1})
	for i := 1; i < numRows; i++ {
		row := make([]int, i+1)
		row[0] = 1
		row[i] = 1
		for j := 1; j < i; j++ {
			row[j] = triangle[i-1][j-1]+triangle[i-1][j]
		}
		triangle = append(triangle, row)
	}
	return triangle
}

func main() {
	fmt.Println(generate(5))
	fmt.Println(generate(1))
}
