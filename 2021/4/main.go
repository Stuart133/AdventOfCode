package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
	"strings"
)

type entry struct {
	value  int
	called bool
}

type board [][]entry

func main() {
	c, err := ioutil.ReadFile("./data.txt")
	if err != nil {
		fmt.Printf("Error opening file: %v", err)
		os.Exit(1)
	}

	rawData := strings.Split(string(c), "\r\n")

	_ = strings.Split(rawData[0], ",")
	boards := make([]board, 0)
	for i := 2; i < len(rawData); i += 6 {
		boards = append(boards, loadBoard(rawData[i:i+5], 5))
	}

	fmt.Println(boards[0])
}

func loadBoard(d []string, s int) board {
	board := make(board, s)
	for i := range board {
		board[i] = make([]entry, s)
	}

	for i := 0; i < s; i++ {
		for j := 0; j < s; j++ {
			line := filter(strings.Split(d[i], " "), "")
			board[i][j] = entry{
				value:  getInt(line[j]),
				called: false,
			}
		}
	}

	return board
}

func filter(s []string, f string) []string {
	out := make([]string, 0)

	for i := range s {
		if s[i] != f {
			out = append(out, s[i])
		}
	}

	return out
}

func getInt(s string) int {
	i, _ := strconv.Atoi(s)

	return i
}
