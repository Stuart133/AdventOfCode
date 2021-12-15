package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
	"strings"
)

type point struct {
	x int
	y int
}

func newPoint(x, y int) point {
	return point{
		x: x,
		y: y,
	}
}

func main() {
	c, err := ioutil.ReadFile("./data-smol.txt")
	if err != nil {
		fmt.Printf("Error opening file: %v", err)
		os.Exit(1)
	}

	rawData := strings.Split(string(c), "\r\n")
	g, _ := buildGrid(rawData, 5)

	fmt.Println(g)
	// risk := search(g, uv, 0, 0, len(g)-1, len(g[0])-1)

	// fmt.Println(risk)
}

func search(g [][]int, uv map[point]int, x, y, dx, dy int) int {
	uv[newPoint(x, y)] = 0
	current := newPoint(x, y)

	for {
		if current.x == dx && current.y == dy {
			return uv[current]
		}

		_, v := uv[newPoint(current.x-1, current.y)]
		if current.x != 0 && v {
			uv[newPoint(current.x-1, current.y)] = min(uv[current]+g[current.x-1][current.y], uv[newPoint(current.x-1, current.y)])
		}
		_, v = uv[newPoint(current.x, current.y-1)]
		if current.y != 0 && v {
			uv[newPoint(current.x, current.y-1)] = min(uv[current]+g[current.x][current.y-1], uv[newPoint(current.x, current.y-1)])
		}
		_, v = uv[newPoint(current.x+1, current.y)]
		if current.x <= len(g)-1 && v {
			uv[newPoint(current.x+1, current.y)] = min(uv[current]+g[current.x+1][current.y], uv[newPoint(current.x+1, current.y)])
		}
		_, v = uv[newPoint(current.x, current.y+1)]
		if current.y <= len(g[0])-1 && v {
			uv[newPoint(current.x, current.y+1)] = min(uv[current]+g[current.x][current.y+1], uv[newPoint(current.x, current.y+1)])
		}

		delete(uv, current)
		current = findMin(uv)
	}
}

func buildGrid(data []string, f int) ([][]int, map[point]int) {
	grid := make([][]int, len(data)*f)
	unvisited := make(map[point]int)

	for k := 0; k < f; k++ {
		for i := range data {
			grid[i+(len(data)*k)] = make([]int, len(data[i])*f)
			for l := 0; l < f; l++ {
				for j := range data[i] {
					rawVal := getInt(string(data[i][j]))
					grid[i+(len(data)*k)][j+len(data[0])*l] = (rawVal + k + l) % 9
					unvisited[newPoint(i+(len(data)*k), j+len(data[0])*l)] = 9999999999999
				}
			}
		}
	}

	return grid, unvisited
}

func min(x, y int) int {
	if x < y {
		return x
	}

	return y
}

func findMin(uv map[point]int) point {
	var minP point
	min := 99999999999999

	for k, v := range uv {
		if v < min {
			min = v
			minP = k
		}
	}

	return minP
}

func getInt(s string) int {
	i, _ := strconv.Atoi(s)

	return i
}
