package main

import (
	"fmt"
	"io/ioutil"
	"math"
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
	total := 0
	for _, l := range lines {
		seg := mapSegments(l)
		lineTotal := 0
		for i, o := range l.output {
			lineTotal += powInt(10, len(l.output)-i-1) * getNumber(seg, o)
		}

		total += lineTotal
	}

	fmt.Println(total)
}

func getNumber(segmentMap map[byte]byte, seg segment) int {
	_, v := seg[segmentMap['g']]
	if v {
		_, v = seg[segmentMap['e']]
		if v {
			if len(seg) == 7 {
				return 8
			} else if len(seg) == 5 {
				return 2
			} else {
				_, v = seg[segmentMap['d']]
				if v {
					return 6
				} else {
					return 0
				}
			}
		} else {
			if len(seg) == 6 {
				return 9
			} else {
				_, v = seg[segmentMap['b']]
				if v {
					return 5
				} else {
					return 3
				}
			}
		}
	} else {
		if len(seg) == 2 {
			return 1
		} else if len(seg) == 4 {
			return 4
		} else {
			return 7
		}
	}
}

func mapSegments(l line) map[byte]byte {
	four := segment{}
	two := segment{}
	freq := make(map[byte]int)
	for _, s := range l.segments {
		for k := range s {
			freq[k]++
		}

		if len(s) == 4 {
			four = s
		}

		if len(s) == 2 {
			two = s
		}
	}

	segmentMap := make(map[byte]byte)
	for k, v := range freq {
		switch v {
		case 4:
			segmentMap['e'] = k
		case 6:
			segmentMap['b'] = k
		case 7:
			_, v := four[k]
			if v {
				segmentMap['d'] = k
			} else {
				segmentMap['g'] = k
			}
		case 8:
			_, v := two[k]
			if v {
				segmentMap['c'] = k
			} else {
				segmentMap['a'] = k
			}
		case 9:
			segmentMap['f'] = k
		}
	}

	return segmentMap
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

func powInt(x, y int) int {
	return int(math.Pow(float64(x), float64(y)))
}
