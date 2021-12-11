package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
	"strings"
)

func main() {
	c, err := ioutil.ReadFile("./data-smol.txt")
	if err != nil {
		fmt.Printf("Error opening file: %v", err)
		os.Exit(1)
	}
	rawData := strings.Split(string(c), "\r\n")
	grid := buildGrid(rawData)

	for s := 0; true; s++ {
		flashes := tick(grid)
		if flashes == len(grid)*len(grid[0]) {
			println(s)
			break
		}
	}
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

func tick(grid [][]int) int {
	for i := range grid {
		for j := range grid[i] {
			grid[i][j]++
			if grid[i][j] == 10 {
				flash(grid, i, j)
			}
		}
	}

	flashes := resetFlashed(grid)
	return flashes
}

func flash(grid [][]int, x, y int) {
	for i := -1; i <= 1; i++ {
		for j := -1; j <= 1; j++ {
			if x+i < 0 || y+j < 0 || x+i >= len(grid) || y+j >= len(grid[x+i]) || (i == 0 && j == 0) {
				continue
			}

			grid[x+i][y+j]++
			if grid[x+i][y+j] == 10 {
				flash(grid, x+i, y+j)
			}
		}
	}
}

func resetFlashed(grid [][]int) int {
	flashes := 0
	for i := range grid {
		for j := range grid[i] {
			if grid[i][j] > 9 {
				grid[i][j] = 0
				flashes++
			}
		}
	}

	return flashes
}

func printGrid(grid [][]int) {
	for i := range grid {
		fmt.Println(grid[i])
	}
	fmt.Println()
}

func getInt(s string) int {
	i, _ := strconv.Atoi(s)

	return i
}
