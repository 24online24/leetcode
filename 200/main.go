package main

import "fmt"

func numIslands(grid [][]byte) int {
	islands := 0
	m := len(grid)
	n := len(grid[0])
	for i := range m {
		for j := range n {
			if grid[i][j] == 0 {
				continue
			}
			
		}
	}
}

func main() {
	fmt.Println(numIslands([][]byte{[]byte{1, 1, 1, 1, 0}, []byte{1, 1, 0, 1, 0}, []byte{1, 1, 0, 0, 0}, []byte{0, 0, 0, 0, 0}}))
}
