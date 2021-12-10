package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"sort"
	"strings"
)

type stack struct {
	data []byte
	head int
}

func main() {
	c, err := ioutil.ReadFile("./data.txt")
	if err != nil {
		fmt.Printf("Error opening file: %v", err)
		os.Exit(1)
	}

	brackets := map[byte]byte{
		'{': '}',
		'[': ']',
		'(': ')',
		'<': '>',
	}
	score := map[byte]int{
		')': 3,
		']': 57,
		'}': 1197,
		'>': 25137,
	}
	completionScore := map[byte]int{
		')': 1,
		']': 2,
		'}': 3,
		'>': 4,
	}

	total := 0
	compScores := make([]int, 0)
	lines := strings.Split(string(c), "\r\n")
	for i := range lines {
		v, c := checkCorrupt(lines[i], brackets)
		if !v {
			total += score[c]
		} else {
			compScores = append(compScores, findCompletion(lines[i], brackets, completionScore))
		}
	}

	fmt.Println(total)
	fmt.Println(getCompletionScore(compScores))
}

func getCompletionScore(scores []int) int {
	sort.Ints(scores)

	return scores[(len(scores) / 2)]
}

func findCompletion(l string, brack map[byte]byte, score map[byte]int) int {
	s := stack{}
	for i := range l {
		if l[i] == '{' || l[i] == '(' || l[i] == '[' || l[i] == '<' {
			s.push(l[i])
		} else {
			s.pop()
		}
	}

	complete := 0
	for s.size() != 0 {
		c := s.pop()
		comp := brack[c]

		complete = complete * 5
		complete += score[comp]
	}

	return complete
}

func checkCorrupt(l string, brack map[byte]byte) (bool, byte) {
	s := stack{}
	for i := range l {
		if l[i] == '{' || l[i] == '(' || l[i] == '[' || l[i] == '<' {
			s.push(l[i])
		} else {
			expect := s.pop()
			if brack[expect] != l[i] {
				return false, l[i]
			}
		}
	}

	return true, 0
}

func (s *stack) push(i byte) {
	s.data = append(s.data, i)
	s.head++
}

func (s *stack) pop() byte {
	val := s.data[s.head-1]
	s.data = s.data[:len(s.data)-1]
	s.head--

	return val
}

func (s *stack) size() int {
	return s.head
}
