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
	points := make([]point, 0)
	dist := (l.one.x - l.two.x) + (l.one.y - l.two.y)
	if dist < 0 {
		dist = (-dist)
	}
	dist++

	if l.one.x == l.two.x {
		startY := 0
		if l.one.y > l.two.y {
			startY = l.two.y
		} else {
			startY = l.one.y
		}
		for i := 0; i < dist; i++ {
			points = append(points, point{
				x: l.one.x,
				y: startY + i,
			})
		}
	} else if l.one.y == l.two.y {
		startX := 0
		if l.one.x > l.two.x {
			startX = l.two.x
		} else {
			startX = l.one.x
		}
		for i := 0; i < dist; i++ {
			points = append(points, point{
				x: startX + i,
				y: l.one.y,
			})
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

	data := strings.Split(string(c), "\n")
	lines := make([]line, len(data))
	for i := range data {
		lines[i] = parseLine(data[i])
	}

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
