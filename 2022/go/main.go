package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func main() {
	data, err := ioutil.ReadFile("data.txt")
	if err != nil {
		fmt.Printf("error readingg file: %s", err)
		return
	}

	len := 0
	order := make([]*listNode, 0)
	head := &listNode{}
	current := head
	key := 811589153
	for _, line := range strings.Split(string(data), "\n") {
		len++
		current.value, err = strconv.Atoi(line)
		current.value = current.value * key
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
	for i := 0; i < 10; i++ {
		for _, node := range order {
			if node.value == 0 {
				zero = node
				continue
			}

			new_pos := node.move(node.value % (len - 1))

			if new_pos == node {
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
		}
	}

	one := zero.move(1000)
	two := zero.move(2000)
	three := zero.move(3000)

	fmt.Printf("%d", one.value+two.value+three.value)
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
