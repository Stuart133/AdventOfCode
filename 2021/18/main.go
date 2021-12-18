package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
	"strings"
)

type node struct {
	parent *node
	left   *node
	right  *node
	value  int
}

func main() {
	c, err := ioutil.ReadFile("./data-smol.txt")
	if err != nil {
		fmt.Printf("Error opening file: %v\n", err)
		os.Exit(1)
	}

	rawData := strings.Split(string(c), "\n\r")
	for i := range rawData {
		tree := parse(rawData[i])
		inOrder(&tree)
	}

}

func inOrder(t *node) {
	if t == nil {
		return
	}

	inOrder(t.left)
	if t.value != 0 {
		fmt.Println(t.value)
	}
	inOrder(t.right)
}

func parse(s string) node {
	root := node{}
	cur := &root
	for _, c := range s {
		if c == '[' {
			cur.left = &node{
				parent: cur,
			}
			cur = cur.left
		} else if c == ']' {
			cur = cur.parent
		} else if c == ',' {
			cur = cur.parent
			cur.right = &node{
				parent: cur,
			}
			cur = cur.right
		} else {
			cur.value = getInt(string(c))
		}
	}

	return root
}

func getInt(s string) int {
	i, _ := strconv.Atoi(s)

	return i
}
