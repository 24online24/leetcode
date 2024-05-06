package main

import (
	"fmt"
	"slices"
	"strings"
)

func reversePrefix(word string, ch byte) string {
	idx := strings.Index(word, string(ch))
	if idx == -1 {
		return word
	}
	pref := []rune(word[0 : idx+1])
	slices.Reverse(pref)
	return string(pref) + word[idx+1:]
}

// func reversePrefix(word string, ch byte) string {

// }

func main() {
	fmt.Println(reversePrefix("abcdefd", 'd'))
	fmt.Println(reversePrefix("xyxzxe", 'z'))
	fmt.Println(reversePrefix("abcd", 'z'))
}
