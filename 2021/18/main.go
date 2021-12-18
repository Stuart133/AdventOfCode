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
		inOrderExplode(&tree, 0)
		inOrderPrint(&tree)
	}
}

func inOrderPrint(t *node) {
	if t == nil {
		return
	}

	inOrderPrint(t.left)
	if t.value != 0 {
		fmt.Println(t.value)
	}
	inOrderPrint(t.right)
}

func inOrderExplode(t *node, d int) {
	if t == nil {
		return
	}

	if d >= 4 && t.value == 0 && t.left.value != 0 {
		left := prev(t.left)
		if left != nil {
			left.value += t.left.value
		}
		right := next(t.right)
		if right != nil {
			right.value += t.right.value
		}

		t.left = nil
		t.right = nil
	}

	inOrderExplode(t.left, d+1)
	inOrderExplode(t.right, d+1)
}

func next(t *node) *node {
	if t.right != nil {
		return treeMinimum(t.right)
	}

	cur := t.parent
	for cur != nil && t == cur.right {
		t = cur
		cur = cur.parent
	}

	if cur == nil {
		return nil
	} else {
		return treeMaximum(cur.right)
	}
}

func prev(t *node) *node {
	if t.left != nil {
		return treeMaximum(t.left)
	}

	cur := t.parent
	for cur != nil && t == cur.left {
		t = cur
		cur = cur.parent
	}

	if cur == nil {
		return nil
	} else {
		return treeMaximum(cur.left)
	}
}

func treeMinimum(t *node) *node {
	if t.left != nil {
		return treeMinimum(t.left)
	} else if t.right != nil {
		return treeMinimum(t.right)
	}

	return t
}

func treeMaximum(t *node) *node {
	if t.right != nil {
		return treeMaximum(t.right)
	} else if t.left != nil {
		return treeMaximum(t.left)
	}

	return t
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
