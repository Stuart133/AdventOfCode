package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
	"strings"
)

type instruction struct {
	x1 int
	x2 int
	y1 int
	y2 int
	z1 int
	z2 int
	on bool
}

type reactor struct {
	grid  [][][]bool
	count int
}

func main() {
	c, err := ioutil.ReadFile("./data-smol.txt")
	if err != nil {
		fmt.Printf("Error opening file: %v\n", err)
		os.Exit(1)
	}

	rawData := strings.Split(string(c), "\n")
	in := make([]instruction, len(rawData))
	for i := range rawData {
		in[i] = parse(rawData[i])
	}

	g := makeGrid(100001)

	for i := range in {
		step(&g, in[i])
	}

	fmt.Println(count(g))
}

func step(g *reactor, in instruction) {
	for i := in.x1; i <= in.x2 && i < 100001 && i >= 0; i++ {
		for j := in.y1; j <= in.y2 && j < 100001 && j >= 0; j++ {
			for k := in.z1; k <= in.z2 && k < 100001 && k >= 0; k++ {
				if g.grid[i][j][k] != in.on {
					if in.on {
						g.count++
					} else {
						g.count--
					}
				}
				g.grid[i][j][k] = in.on
			}
		}
	}
}

func count(g reactor) int {
	c := 0
	for i := range g.grid {
		for j := range g.grid[i] {
			for k := range g.grid[i][j] {
				if g.grid[i][j][k] {
					c++
				}
			}
		}
	}

	return c
}

func parse(s string) instruction {
	i := instruction{}
	d := strings.Split(s, " ")
	i.on = d[0] == "on"

	n := strings.Split(d[1], ",")

	x := strings.Split(n[0], "..")
	x1 := getInt(x[0][2:])
	x2 := getInt(x[1])
	i.x1 = min(x1, x2)
	i.x2 = max(x1, x2)

	y := strings.Split(n[1], "..")
	y1 := getInt(y[0][2:])
	y2 := getInt(y[1])
	i.y1 = min(y1, y2)
	i.y2 = max(y1, y2)

	z := strings.Split(n[2], "..")
	z1 := getInt(z[0][2:])
	z2 := getInt(z[1])
	i.z1 = min(z1, z2)
	i.z2 = max(z1, z2)

	return i
}

func makeGrid(n int) reactor {
	r := reactor{}
	r.grid = make([][][]bool, n)
	for i := range r.grid {
		r.grid[i] = make([][]bool, n)
		for j := range r.grid {
			r.grid[i][j] = make([]bool, n)
		}
	}

	return r
}

func max(a, b int) int {
	if a > b {
		return a
	}

	return b
}

func min(a, b int) int {
	if a < b {
		return a
	}

	return b
}

func getInt(s string) int {
	i, _ := strconv.Atoi(s)

	return i
}
