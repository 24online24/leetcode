package main

import "fmt"

func checkIfPangram(sentence string) bool {
    alphabet := make([]bool, 26)
    var result bool = true
    for _, character := range sentence {
        alphabet[character-'a'] = true
    }
    for _, element := range alphabet {
        result = result && element
    }
    return result
}

func main() {
    fmt.Println(checkIfPangram("thequickbrownfoxjumpsoverthelazydog"))
    fmt.Println(checkIfPangram("leetcode"))
}
