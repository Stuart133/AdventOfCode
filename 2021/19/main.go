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

	_ = parse(strings.Split(string(c), "\n"))
	// fmt.Println(scanners[0])
}

func parse(s []string) []scanner {
	scanner := make([]scanner, 0)

	start := 0
	for i := range s {
		if len(s[i]) == 0 {
			scanner = append(scanner, parseScanner(s[start:i]))
			fmt.Println(scanner)
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
