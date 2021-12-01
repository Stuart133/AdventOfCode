package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
	"strings"
)

func main() {
	c, err := ioutil.ReadFile("./data.txt")
	if err != nil {
		fmt.Printf("Error opening file: %v", err)
		os.Exit(1)
	}

	rawData := strings.Split(string(c), "\n")
	data := make([]int, len(rawData))
	for i := range rawData {
		data[i], _ = strconv.Atoi(strings.TrimSpace(rawData[i]))
	}

	threeCount := calcThreeWindow(data)

	fmt.Println(threeCount)
}

func calcThreeWindow(data []int) int {
	sums := make([]int, 3)
	c := 0

	// Preload sums
	sums[0] = data[0] + data[1] + data[2]
	sums[1] = data[1] + data[2]
	sums[2] = data[2]

	for i := 3; i < len(data); i++ {
		n := mod(i+1, 3)
		sums[n] += data[i]

		// Compare sums
		if sums[mod(n-1, 3)] < sums[n] {
			c++
		}

		// Reset trailing sum
		sums[mod(n-1, 3)] = 0

		// Add remaining sums
		sums[mod(n-1, 3)] += data[i]
		sums[mod(n+1, 3)] += data[i]
	}

	return c
}

func calculateIncreases(data []int) int {
	c := 0
	prev := data[0]

	for i := range data {
		if i == 0 {
			continue
		}

		if data[i] > prev {
			c++
		}

		prev = data[i]
	}

	return c
}

func mod(a, b int) int {
	return (a%b + b) % b
}
