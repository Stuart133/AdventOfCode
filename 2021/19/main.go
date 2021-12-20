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
	z int
}

type scanner []point

func main() {
	c, err := ioutil.ReadFile("./data-smol.txt")
	if err != nil {
		fmt.Printf("Error opening file: %v\n", err)
		os.Exit(1)
	}

	scanner := parse(strings.Split(string(c), "\n"))

	x := make(map[int]int)
	y := make(map[int]int)
	z := make(map[int]int)
	for i := range scanner[0] {
		for j := range scanner[1] {
			x1, x2, y1, y2, z1, z2 := getDiff(scanner[0][i], scanner[1][j])
			x[x1]++
			x[x2]++
			x[y1]++
			x[y2]++
			x[z1]++
			x[z2]++
			if x[x1] > 10 {
				fmt.Println(x1)
			}
			if x[x2] > 10 {
				fmt.Println(x2)
			}
			if y[y1] > 10 {
				fmt.Println(y1)
			}
			if y[y2] > 10 {
				fmt.Println(y2)
			}
			if z[z1] > 10 {
				fmt.Println(z1)
			}
			if z[z2] > 10 {
				fmt.Println(z2)
			}
		}
	}
}

func getDiff(a, b point) (int, int, int, int, int, int) {
	x1 := abs(a.x - b.x)
	x2 := abs(a.x + b.x)
	y1 := abs(a.y - b.y)
	y2 := abs(a.y + b.y)
	z1 := abs(a.z - b.z)
	z2 := abs(a.z + b.z)

	return x1, x2, y1, y2, z1, z2
}

func parse(s []string) []scanner {
	scanner := make([]scanner, 0)

	start := 0
	for i := range s {
		if len(s[i]) == 0 {
			scanner = append(scanner, parseScanner(s[start:i]))
		} else if s[i][0:2] == "--" {
			start = i + 1
		}
	}

	return scanner
}

func parseScanner(s []string) scanner {
	scanner := make(scanner, len(s))
	for i := range s {
		scanner[i] = parsePoint(s[i])
	}

	return scanner
}

func parsePoint(s string) point {
	n := strings.Split(s, ",")
	return point{
		x: getInt(n[0]),
		y: getInt(n[1]),
		z: getInt(n[2]),
	}
}

func getInt(s string) int {
	n, _ := strconv.Atoi(s)

	return n
}

func abs(n int) int {
	if n < 0 {
		return -n
	}

	return n
}
