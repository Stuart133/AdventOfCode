package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	c, err := ioutil.ReadFile("./data.txt")
	if err != nil {
		fmt.Printf("Error opening file: %v", err)
		os.Exit(1)
	}

	rawData := strings.Split(string(c), "\r\n")
	grid := buildGrid(rawData)
	total := totalLowPoints(grid)
	fmt.Println(total)

	total = findLargestBasins(grid, 3)
	fmt.Println(total)
}

func findLargestBasins(g [][]int, n int) int {
	basins := make([]int, 0)

	for i := range g {
		for j := range g[i] {
			basin := findBasin(g, i, j)
			if basin != 0 {
				basins = append(basins, basin)
			}
		}
	}

	sort.Ints(basins)
	total := 1
	for i := len(basins) - n; i < len(basins); i++ {
		total *= basins[i]
	}

	return total
}

func findBasin(g [][]int, i, j int) int {
	if i < 0 || j < 0 || i >= len(g) || j >= len(g[i]) || g[i][j] == 9 {
		return 0
	}

	g[i][j] = 9
	basinSize := 1

	basinSize += findBasin(g, i-1, j)
	basinSize += findBasin(g, i, j-1)
	basinSize += findBasin(g, i+1, j)
	basinSize += findBasin(g, i, j+1)

	return basinSize
}

func totalLowPoints(g [][]int) int {
	total := 0

	for i := range g {
		for j := range g[i] {
			low := true

			// Up
			if i-1 >= 0 {
				low = low && (g[i][j] < g[i-1][j])
			}

			// Left
			if j-1 >= 0 {
				low = low && (g[i][j] < g[i][j-1])
			}

			// Down
			if i+1 < len(g) {
				low = low && (g[i][j] < g[i+1][j])
			}

			// Right
			if j+1 < len(g[i]) {
				low = low && (g[i][j] < g[i][j+1])
			}

			if low {
				total += g[i][j] + 1
			}
		}
	}

	return total
}

func buildGrid(data []string) [][]int {
	grid := make([][]int, len(data))

	for i := range data {
		grid[i] = make([]int, len(data[i]))
		for j := range data[i] {
			grid[i][j] = getInt(string(data[i][j]))
		}
	}

	return grid
}

func getInt(s string) int {
	i, _ := strconv.Atoi(s)

	return i
}
