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

	rawData := strings.Split(string(c), "\n")
	var tree *node
	for i := range rawData {
		if i == 0 {
			tree = parse(rawData[i])
			continue
		}
		add := parse(rawData[i])
		tree = addTrees(tree, add)
		reduce(tree)
	}

	inOrderPrint(tree)
}

func reduce(t *node) {
	for {
		ex := inOrderExplode(t, 0)
		if ex {
			continue
		}

		split := inOrderSplit(t)
		if split {
			continue
		}

		break
	}
}

func addTrees(a *node, b *node) *node {
	new := &node{
		left:  a,
		right: b,
	}

	a.parent = new
	b.parent = new

	return new
}

func inOrderPrint(t *node) {
	if t == nil {
		return
	}

	inOrderPrint(t.left)
	if t.left == nil {
		fmt.Println(t.value)
	}
	inOrderPrint(t.right)
}

func inOrderSplit(t *node) bool {
	if t == nil {
		return false
	}

	split := false
	if t.value >= 10 {
		leftVal := t.value / 2
		rightVal := 0
		if t.value%2 == 0 {
			rightVal = t.value / 2
		} else {
			rightVal = (t.value / 2) + 1
		}
		split = true
		t.left = &node{
			parent: t,
			value:  leftVal,
		}
		t.right = &node{
			parent: t,
			value:  rightVal,
		}
		t.value = 0

		return split
	}

	split = inOrderSplit(t.left) || split
	if split {
		return split
	}
	split = inOrderSplit(t.right) || split

	return split
}

func inOrderExplode(t *node, d int) bool {
	if t == nil {
		return false
	}

	exploded := false
	if d >= 4 && t.left != nil && t.left.left == nil {
		exploded = true
		left := prev(t.left)
		if left != nil {
			left.value += t.left.value
		}
		right := next(t.right)
		if right != nil {
			right.value += t.right.value
		}

		t.value = 0
		t.left = nil
		t.right = nil

		return exploded
	}

	exploded = inOrderExplode(t.left, d+1) || exploded
	if exploded {
		return exploded
	}
	exploded = inOrderExplode(t.right, d+1) || exploded

	return exploded
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
		return treeMinimum(cur.right)
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

func parse(s string) *node {
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

	return &root
}

func getInt(s string) int {
	i, _ := strconv.Atoi(s)

	return i
}
