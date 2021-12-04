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

type board struct {
	nums [][]entry
	won  bool
}

func main() {
	c, err := ioutil.ReadFile("./data.txt")
	if err != nil {
		fmt.Printf("Error opening file: %v", err)
		os.Exit(1)
	}

	rawData := strings.Split(string(c), "\n")

	calls := strings.Split(rawData[0], ",")
	boards := make([]board, 0)
	for i := 2; i < len(rawData); i += 6 {
		boards = append(boards, loadBoard(rawData[i:i+5], 5))
	}

	lastWinningBoard := -1
	totalWon := 0
	for i := range calls {
		if totalWon == 100 {
			break
		}
		for j := range boards {
			if boards[j].won {
				continue
			}
			callNumber(boards[j], getInt(calls[i]))
			if checkBoard(boards[j]) {
				boards[j].won = true
				totalWon++
				lastWinningBoard = j
				fmt.Printf("Got a winning board %d - Number %s\n", j, calls[i])
			}
		}
	}

	fmt.Printf("Winning total: %d. Total won %d\n", totalUncalled(boards[lastWinningBoard]), totalWon)
}

func callNumber(b board, n int) {
	for i := range b.nums {
		for j := range b.nums[i] {
			if b.nums[i][j].value == n {
				b.nums[i][j].called = true
				return
			}
		}
	}
}

func checkBoard(b board) bool {
	vert := make([]bool, len(b.nums))
	for i := range vert {
		vert[i] = true
	}
	horizontal := true

	for i := range b.nums {
		for j := range b.nums[i] {
			horizontal = horizontal && b.nums[i][j].called
			vert[j] = vert[j] && b.nums[i][j].called
		}

		if horizontal {
			return true
		}
		horizontal = true
	}

	for i := range vert {
		if vert[i] {
			return true
		}
	}

	return false
}

func totalUncalled(b board) int {
	total := 0
	for i := range b.nums {
		for j := range b.nums[i] {
			if !b.nums[i][j].called {
				total += b.nums[i][j].value
			}
		}
	}

	return total
}

func loadBoard(d []string, s int) board {
	board := board{}
	board.nums = make([][]entry, s)
	for i := range board.nums {
		board.nums[i] = make([]entry, s)
	}

	for i := 0; i < s; i++ {
		for j := 0; j < s; j++ {
			line := filter(strings.Split(d[i], " "), "")
			board.nums[i][j] = entry{
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
