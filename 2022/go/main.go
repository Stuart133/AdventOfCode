package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func main() {
	data, err := ioutil.ReadFile("data_smol.txt")
	if err != nil {
		fmt.Printf("error readnig file: %s", err)
		return
	}

	order := make([]*listNode, 0)
	head := &listNode{}
	current := head
	for _, line := range strings.Split(string(data), "\n") {
		current.value, err = strconv.Atoi(line)
		if err != nil {
			fmt.Println(err)
		}
		order = append(order, current)

		next := listNode{
			prev: current,
		}

		current.next = &next
		current = current.next
	}

	// Make it circular
	tail := current.prev
	head.prev = tail
	tail.next = head

	var zero *listNode
	for _, node := range order {
		if node.value == 0 {
			zero = node
			continue
		}

		if node == head {
			head = node.next
		}

		new_pos := node.move(node.value)

		if new_pos == node {
			fmt.Println("I told you so")
			continue
		}

		// Splice out old node
		node.next.prev = node.prev
		node.prev.next = node.next

		// Splice in the new node
		if node.value > 0 {
			temp := new_pos.next
			new_pos.next = node
			temp.prev = node

			node.next = temp
			node.prev = new_pos
		} else {
			temp := new_pos.prev
			new_pos.prev = node
			temp.next = node

			node.prev = temp
			node.next = new_pos
		}

		// head.print(7)
		// fmt.Println()
	}

	one := zero.move(1000)
	two := one.move(1000)
	three := two.move(1000)

	fmt.Println(one.value + two.value + three.value)
}

type listNode struct {
	next  *listNode
	prev  *listNode
	value int
}

func (n *listNode) move(count int) *listNode {
	current := n
	if count > 0 {
		for i := 0; i < count; i++ {
			current = current.next
		}
	} else {
		for i := 0; i > count; i-- {
			current = current.prev
		}
	}

	return current
}

func (n *listNode) print(len int) {
	current := n
	for i := 0; i < len; i++ {
		fmt.Print(current.value, " ")
		current = current.next
	}
}
