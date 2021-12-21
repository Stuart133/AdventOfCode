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

type state struct {
	p1       player
	p2       player
	p1Active bool
}

func (s state) next(roll int) state {
	if s.p1Active {
		s.p1.position = mod(s.p1.position+roll, 10)
		s.p1.score += s.p1.position
		s.p1Active = false
	} else {
		s.p2.position = mod(s.p2.position+roll, 10)
		s.p2.score += s.p2.position
		s.p1Active = true
	}

	return s
}

func turn(s state) (int64, int64) {
	if s.p1.score >= 21 {
		return 1, 0
	} else if s.p2.score >= 21 {
		return 0, 1
	}

	ow, tw := int64(0), int64(0)
	for i := 1; i <= 3; i++ {
		for j := 1; j <= 3; j++ {
			for k := 1; k <= 3; k++ {
				o, t := turn(s.next(i + j + k))
				ow += o
				tw += t
			}
		}
	}

	return ow, tw
}
func main() {
	c, err := ioutil.ReadFile("./data-smol.txt")
	if err != nil {
		fmt.Printf("Error opening file: %v\n", err)
		os.Exit(1)
	}

	rawData := strings.Split(string(c), "\n")

	p1 := parse(rawData[0])
	p2 := parse(rawData[1])
	s := state{
		p1:       p1,
		p2:       p2,
		p1Active: true,
	}

	ow, tw := turn(s)
	fmt.Println(ow)
	fmt.Println(tw)
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
