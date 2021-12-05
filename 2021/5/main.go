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

func main() {
	c, err := ioutil.ReadFile("./data.txt")
	if err != nil {
		fmt.Printf("Error opening file: %v", err)
		os.Exit(1)
	}

	data := strings.Split(string(c), "\n")

}

func parseLine(s string) line {
	points := strings.Split(s, " -> ")

	p1 := strings.Split(points[0], ",")
	p2 := strings.Split(points[1], ",")

	return line{
		one: point{
			x: getInt(p1[0]),
			y: getInt(p2[0]),
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
