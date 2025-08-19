package main

func matrixScore(grid [][]int) int {
	m, n := len(grid), len(grid[0])
	score := 1 << (n - 1) * m
	for j := 1; j < n; j++ {
		equalToFirst := 0
		for i := range m {
			if grid[i][j] == grid[i][0] {
				equalToFirst++
			}
		}
		dif := m - equalToFirst
		if equalToFirst > dif {
			score += 1 << (n - 1 - j) * equalToFirst
		} else {
			score += 1 << (n - 1 - j) * dif
		}
	}
	return score
}

func main() {

}
