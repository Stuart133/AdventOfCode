package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
	"strings"
)

type player struct {
	position int
	score    int
}

func (p *player) turn(d *die) {
	move := 0
	for i := 0; i < 3; i++ {
		move += d.roll()
	}

	p.position = mod(p.position+move, 10)
	p.score += p.position
}

type die struct {
	value int
}

func (d *die) roll() int {
	ret := d.value
	d.value = mod(d.value+1, 100)

	return ret
}

func main() {
	c, err := ioutil.ReadFile("./data.txt")
	if err != nil {
		fmt.Printf("Error opening file: %v\n", err)
		os.Exit(1)
	}

	rawData := strings.Split(string(c), "\n")
	p1 := parse(rawData[0])
	p2 := parse(rawData[1])
	die := die{
		value: 1,
	}

	rollCount := 0
	for {
		if p2.score >= 1000 {
			break
		}

		p1.turn(&die)
		rollCount += 3

		if p1.score >= 1000 {
			break
		}

		p2.turn(&die)
		rollCount += 3
	}

	losingScore := 0
	if p1.score >= 1000 {
		losingScore = p2.score
	} else {
		losingScore = p1.score
	}

	fmt.Println(losingScore * rollCount)
}

func parse(s string) player {
	d := strings.Split(s, " ")
	p := player{
		position: getInt(d[len(d)-1]),
	}

	return p
}

func getInt(s string) int {
	i, _ := strconv.Atoi(s)

	return i
}

func mod(i, m int) int {
	ret := i % m
	if ret == 0 {
		return m
	}

	return ret
}
