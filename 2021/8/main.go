package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strings"
)

type segment map[byte]interface{}

type line struct {
	segments []segment
	output   []segment
}

func main() {
	c, err := ioutil.ReadFile("./data.txt")
	if err != nil {
		fmt.Printf("Error opening file: %v", err)
		os.Exit(1)
	}

	rawData := strings.Split(string(c), "\r\n")
	lines := make([]line, len(rawData))
	for i := range rawData {
		lines[i] = processLine(rawData[i])
	}

	fmt.Println(countUnique(lines))
}

func countUnique(lines []line) int {
	total := 0
	for i := range lines {
		for _, o := range lines[i].output {
			if len(o) == 2 || len(o) == 3 || len(o) == 4 || len(o) == 7 {
				total++
			}
		}
	}

	return total
}

func processLine(l string) line {
	line := line{}

	sl := strings.Split(l, " | ")
	for i, s := range strings.Split(sl[0], " ") {
		line.segments = append(line.segments, segment{})
		for j := range s {
			line.segments[i][s[j]] = nil
		}
	}

	for i, s := range strings.Split(sl[1], " ") {
		line.output = append(line.output, segment{})
		for j := range s {
			line.output[i][s[j]] = nil
		}
	}

	return line
}
