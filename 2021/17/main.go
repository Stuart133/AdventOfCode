package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
	"strings"
)

type probe struct {
	x  int
	y  int
	dx int
	dy int
}

func (p *probe) step() {
	p.x += p.dx
	p.y += p.dy

	if p.dx != 0 {
		p.dx--
	}
	p.dy--
}

type trench struct {
	xStart int
	xEnd   int
	yStart int
	yEnd   int
}

func main() {
	c, err := ioutil.ReadFile("./data.txt")
	if err != nil {
		fmt.Printf("Error opening file: %v\n", err)
		os.Exit(1)
	}

	t := loadTrench(string(c))
	fmt.Println(calcMaxHeight(t))
	fmt.Println(calcPossible(t))
}

func calcPossible(trench trench) int {
	count := 0

	for i := -750; i < 750; i++ {
		for j := -750; j < 750; j++ {
			p := probe{
				x:  0,
				y:  0,
				dx: i,
				dy: j,
			}
			for _ = 0; true; p.step() {
				if (p.x >= trench.xStart && p.x <= trench.xEnd) &&
					(p.y >= trench.yStart && p.y <= trench.yEnd) {
					count++
					break
				}

				if p.x > trench.xEnd || p.y < trench.yStart {
					break
				}
			}
		}
	}

	return count
}

func calcMaxHeight(t trench) int {
	maxH := 0
	for i := 0; i < 200; i++ {
		h := (i * (i + 1)) / 2
		for j := 0; true; j++ {
			if h-((j*(j+1))/2) >= t.yStart && h-((j*(j+1))/2) <= t.yEnd {
				maxH = h
			}

			if h-((j*(j+1))/2) < t.yEnd {
				break
			}
		}
	}

	return maxH
}

func loadTrench(data string) trench {
	d := strings.Split(data, " ")
	x := strings.Split(strings.Split(d[2], "=")[1], "..")
	y := strings.Split(strings.Split(d[3], "=")[1], "..")

	return trench{
		xStart: getInt(x[0]),
		xEnd:   getInt(x[1]),
		yStart: getInt(y[0]),
		yEnd:   getInt(y[1]),
	}
}

func getInt(s string) int {
	i, _ := strconv.Atoi(s)

	return i
}
