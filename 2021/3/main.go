package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strings"
)

func main() {
	c, err := ioutil.ReadFile("./data-smol.txt")
	if err != nil {
		fmt.Printf("Error opening file: %v", err)
		os.Exit(1)
	}

	data := strings.Split(string(c), "\r\n")
	powerConsumption(data)
}

func oxygenRating(data []string) {

}

func powerConsumption(data []string) {
	count := make([]int, len(data[0]))
	for i := range data {
		for j := range data[i] {
			switch data[i][j] {
			case '1':
				count[j]++
			case '0':
				count[j]--
			default:
				panic(fmt.Errorf("unexpected input %v", count[j]))
			}
		}
	}

	for i := range count {
		if count[i] > 0 {
			fmt.Printf("1")
		} else {
			fmt.Printf("0")
		}
	}
	fmt.Printf("\n")
	for i := range count {
		if count[i] > 0 {
			fmt.Printf("0")
		} else {
			fmt.Printf("1")
		}
	}
}
