package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strings"
)

const steps = 40

func main() {
	c, err := ioutil.ReadFile("./data-smol.txt")
	if err != nil {
		fmt.Printf("Error opening file: %v", err)
		os.Exit(1)
	}

	p, pairs := loadPolymer(strings.Split(string(c), "\r\n"))
	freq := solve(p, pairs)

	fmt.Println(getScore(freq))
}

func solve(polymer string, pairs map[string]string) map[string]int {
	letters := make(map[string]int)
	for i := 0; i < len(polymer)-1; i++ {
		subLetter := letterFreq(polymer[i:i+2], pairs, 0, false)
		for k, v := range subLetter {
			letters[k] += v
		}
	}

	letters[string(polymer[len(polymer)-1])]++
	return letters
}

func letterFreq(polymer string, pairs map[string]string, depth int, isLeft bool) map[string]int {
	letters := make(map[string]int)
	if !isLeft {
		for i := 0; i < len(polymer)-1; i++ {
			letters[string(polymer[i])]++
		}
	}

	if depth == steps {
		return letters
	}

	insert := pairs[polymer]
	left := letterFreq(fmt.Sprintf("%s%s", string(polymer[0]), insert), pairs, depth+1, true)
	right := letterFreq(fmt.Sprintf("%s%s", insert, string(polymer[1])), pairs, depth+1, false)
	for k, v := range left {
		letters[k] += v
	}
	for k, v := range right {
		letters[k] += v
	}
	return letters
}

func getScore(letters map[string]int) int {
	maxCount := 0
	minCount := 999999999
	for _, v := range letters {
		maxCount = max(maxCount, v)
		minCount = min(minCount, v)
	}

	return maxCount - minCount
}

func loadPolymer(data []string) (string, map[string]string) {
	polymer := data[0]
	pairs := make(map[string]string)

	for i := 2; i < len(data); i++ {
		d := strings.Split(data[i], " -> ")
		pairs[d[0]] = d[1]
	}

	return polymer, pairs
}

func max(x, y int) int {
	if x > y {
		return x
	}

	return y
}

func min(x, y int) int {
	if x < y {
		return x
	}

	return y
}
