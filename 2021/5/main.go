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

type line struct {
	one point
	two point
}

func (l line) getPoints() []point {
	xDir := 0
	if l.one.x < l.two.x {
		xDir = 1
	} else if l.one.x > l.two.x {
		xDir = -1
	}

	yDir := 0
	if l.one.y < l.two.y {
		yDir = 1
	} else if l.one.y > l.two.y {
		yDir = -1
	}
	dist := max(abs(l.one.x-l.two.x), abs(l.one.y-l.two.y)) + 1
	points := make([]point, dist)

	for i := 0; i < dist; i++ {
		points[i] = point{
			x: l.one.x + (i * xDir),
			y: l.one.y + (i * yDir),
		}
	}

	return points
}

func main() {
	c, err := ioutil.ReadFile("./data.txt")
	if err != nil {
		fmt.Printf("Error opening file: %v", err)
		os.Exit(1)
	}

	data := strings.Split(string(c), "\r\n")
	lines := make([]line, len(data))
	for i := range data {
		lines[i] = parseLine(data[i])
	}

	lines[0].getPoints()

	hot := make(map[point](int))
	total := 0
	for l := range lines {
		for _, p := range lines[l].getPoints() {
			c, v := hot[p]
			if v {
				hot[p]++
				if c == 1 {
					total++
				}
			} else {
				hot[p] = 1
			}
		}
	}

	fmt.Println(total)
}

func parseLine(s string) line {
	points := strings.Split(s, " -> ")

	p1 := strings.Split(points[0], ",")
	p2 := strings.Split(points[1], ",")

	return line{
		one: point{
			x: getInt(p1[0]),
			y: getInt(p1[1]),
		},
		two: point{
			x: getInt(p2[0]),
			y: getInt(p2[1]),
		},
	}
}

func getInt(s string) int {
	i, _ := strconv.Atoi(s)

	return i
}

func abs(x int) int {
	if x < 0 {
		return -x
	}

	return x
}

func max(x, y int) int {
	if x > y {
		return x
	}

	return y
}
