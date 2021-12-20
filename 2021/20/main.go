package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strings"
)

const steps = 50

func main() {
	c, err := ioutil.ReadFile("./data.txt")
	if err != nil {
		fmt.Printf("Error opening file: %v\n", err)
		os.Exit(1)
	}

	rawData := strings.Split(string(c), "\n")
	alg := parseAlg(rawData[0])
	grid := parseImage(rawData[2:])

	outer := 1
	for i := 0; i < steps; i++ {
		if outer == 1 {
			outer = 0
		} else {
			outer = 1
		}
		grid = step(grid, alg, outer)
	}

	fmt.Println(count(grid))
}

func count(g [][]int) int {
	count := 0
	for i := range g {
		for j := range g[i] {
			if g[i][j] == 1 {
				count++
			}
		}
	}

	return count
}

func step(g [][]int, alg []int, outer int) [][]int {
	ng := make([][]int, len(g))
	for i := range ng {
		ng[i] = make([]int, len(g))
	}

	for i := range g {
		for j := range g[i] {
			ng[i][j] = determinePixel(g, i, j, outer, alg)
		}
	}

	return ng
}

func determinePixel(g [][]int, x, y, outer int, alg []int) int {
	shift := 8
	output := 0
	for i := -1; i <= 1; i++ {
		for j := -1; j <= 1; j++ {
			bit := 0
			if x+i < 0 || y+j < 0 || x+i >= len(g) || y+j >= len(g[x]) {
				bit = outer
			} else {
				bit = g[x+i][y+j]
			}

			output |= (bit << shift)
			shift--
		}
	}

	return alg[output]
}

func parseImage(s []string) [][]int {
	// Padding is a bit of a hack
	grid := make([][]int, len(s)+steps*4)
	for i := range grid {
		grid[i] = make([]int, len(s)+steps*4)
	}

	for i := steps * 2; i < len(s)+steps*2; i++ {
		for j := steps * 2; j < len(s)+steps*2; j++ {
			if s[i-(steps*2)][j-(steps*2)] == '#' {
				grid[i][j] = 1
			} else {
				grid[i][j] = 0
			}
		}
	}

	return grid
}

func parseAlg(s string) []int {
	alg := make([]int, 512)
	for i := range s {
		if s[i] == '#' {
			alg[i] = 1
		} else {
			alg[i] = 0
		}
	}

	return alg
}
