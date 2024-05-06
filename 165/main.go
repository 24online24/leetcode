package main

import (
	"fmt"
	"strconv"
	"strings"
)

func getRevisionsArray(version string) (array []int) {
	for _, s := range strings.Split(version, ".") {
		i, err := strconv.Atoi(s)
		if err != nil {
			panic(err)
		}
		array = append(array, i)
	}
	return
}

func compareVersion(version1 string, version2 string) int {
	v1array := getRevisionsArray(version1)
	v2array := getRevisionsArray(version2)
	i := 0
	for ; i < len(v1array) && i < len(v2array); i++ {
		rev1 := v1array[i]
		rev2 := v2array[i]
		switch {
		case rev1 < rev2:
			return -1
		case rev1 > rev2:
			return 1
		}
	}
	for ; i < len(v1array); i++ {
		if v1array[i] != 0 {
			return 1
		}
	}
	for ; i < len(v2array); i++ {
		if v2array[i] != 0 {
			return -1
		}
	}
	return 0
}

func main() {
	fmt.Println(compareVersion("1.01", "1.001"))
	fmt.Println(compareVersion("1.0", "1.0.0"))
	fmt.Println(compareVersion("0.1", "1.1"))
}
