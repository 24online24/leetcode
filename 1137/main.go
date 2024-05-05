package main

import "fmt"

func tribonacci(n int) int {
	tri0, tri1, tri2 := 0, 1, 1
	switch n {
	case 0:
		return tri0
	case 1:
		return tri1
	case 2:
		return tri2
	default:
		for i := 3; i <= n; i++ {
			tri0, tri1, tri2 = tri1, tri2, tri0+tri1+tri2
		}
		return tri2
	}
}

func main() {
	fmt.Println(tribonacci(4))
	fmt.Println(tribonacci(25))
}
