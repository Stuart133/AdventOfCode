package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strings"
)

type graph struct {
	nodes map[string][]string
}

func (g *graph) addEdge(a, b string) {
	g.nodes[a] = append(g.nodes[a], b)
	g.nodes[b] = append(g.nodes[b], a)
}

func (g *graph) addOneWayEdge(a, b string) {
	g.nodes[a] = append(g.nodes[a], b)
}

func buildGraph(data []string) graph {
	g := graph{
		nodes: make(map[string][]string),
	}
	for _, l := range data {
		connection := strings.Split(l, "-")
		if connection[0] == "start" {
			g.addOneWayEdge(connection[0], connection[1])
		} else if connection[1] == "start" {
			g.addOneWayEdge(connection[1], connection[0])
		} else if connection[0] == "end" {
			g.addOneWayEdge(connection[1], connection[0])
		} else if connection[1] == "end" {
			g.addOneWayEdge(connection[0], connection[1])
		} else {
			g.addEdge(connection[0], connection[1])
		}
	}

	return g
}

func (g *graph) searchRoutes() int {
	total := 0
	q := queue{}
	q.enqueue(qNode{
		vertex:         "start",
		visited:        map[string]interface{}{},
		path:           []string{"start"},
		smallCaveTwice: false,
	})

	for len(q) != 0 {
		node := q.dequeue()
		if node.vertex == "end" {
			total++
			continue
		}
		for _, v := range g.nodes[node.vertex] {
			_, in := node.visited[v]
			smallCave := node.smallCaveTwice
			if in && node.smallCaveTwice {
				continue
			} else if in {
				smallCave = true
			}

			newVisited := copy(node.visited)
			newPath := copySlice(node.path)
			if isLower(v) {
				newVisited[v] = nil
			}
			q.enqueue(qNode{
				vertex:         v,
				visited:        newVisited,
				path:           append(newPath, v),
				smallCaveTwice: smallCave,
			})
		}
	}

	return total
}

func copy(m map[string]interface{}) map[string]interface{} {
	newMap := make(map[string]interface{})

	for k, v := range m {
		newMap[k] = v
	}

	return newMap
}

func copySlice(s []string) []string {
	newSlice := make([]string, len(s))

	for i := range s {
		newSlice[i] = s[i]
	}

	return newSlice
}

type qNode struct {
	vertex         string
	visited        map[string]interface{}
	path           []string
	smallCaveTwice bool
}

type queue []qNode

func (q *queue) enqueue(s qNode) {
	*q = append(*q, s)
}

func (q *queue) dequeue() qNode {
	s := (*q)[0]
	*q = (*q)[1:]

	return s
}

func main() {
	c, err := ioutil.ReadFile("./data.txt")
	if err != nil {
		fmt.Printf("Error opening file: %v", err)
		os.Exit(1)
	}

	rawData := strings.Split(string(c), "\n")
	g := buildGraph(rawData)
	fmt.Println(g.searchRoutes())
}

func isLower(s string) bool {
	if s == "end" {
		return false
	}
	r := s[0]

	return r > 96 && r < 123
}
