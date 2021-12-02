package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
	"strings"
)

type instruction struct {
	direction string
	distance  int
}

func main() {
	c, err := ioutil.ReadFile("./data.txt")
	if err != nil {
		fmt.Printf("Error opening file: %v", err)
		os.Exit(1)
	}

	rawData := strings.Split(string(c), "\r\n")
	data := make([]instruction, len(rawData))
	for i := range rawData {
		l := strings.Split(rawData[i], " ")
		d, err := strconv.Atoi(l[1])
		if err != nil {
			fmt.Printf("Conversion error: %v", err)
		}

		data[i] = instruction{
			direction: l[0],
			distance:  d,
		}
	}

	calcNewCourse(data)
}

func calcNewCourse(d []instruction) {
	depth, pos, aim := 0, 0, 0
	for i := range d {
		switch d[i].direction {
		case "forward":
			pos += d[i].distance
			depth += (d[i].distance * aim)
		case "down":
			aim += d[i].distance
		case "up":
			aim -= d[i].distance
		}
		fmt.Println(aim)
	}

	fmt.Println(depth)
	fmt.Println(pos)
}

func calcCourse(d []instruction) {
	depth, pos := 0, 0
	for i := range d {
		switch d[i].direction {
		case "forward":
			pos += d[i].distance
		case "down":
			depth += d[i].distance
		case "up":
			depth -= d[i].distance
		}
	}

	fmt.Println(depth)
	fmt.Println(pos)
}
