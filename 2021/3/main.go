package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strings"
)

func main() {
	c, err := ioutil.ReadFile("./data.txt")
	if err != nil {
		fmt.Printf("Error opening file: %v", err)
		os.Exit(1)
	}

	data := strings.Split(string(c), "\r\n")
	powerConsumption(data)
	fmt.Print("\nPart 2 *****\n\n")
	fmt.Println(oxygenRating(data))
	fmt.Println(co2Rating(data))
}

func co2Rating(data []string) string {
	for i := 0; i < len(data[0]); i++ {
		data = getRating(data, i, func(cmp int) bool {
			return cmp < 0
		})
		if len(data) == 1 {
			return data[0]
		}
	}

	panic("Shouldn't get here")
}

func oxygenRating(data []string) string {
	for i := 0; i < len(data[0]); i++ {
		data = getRating(data, i, func(cmp int) bool {
			return cmp >= 0
		})
		if len(data) == 1 {
			return data[0]
		}
	}

	panic("Shouldn't get here")
}

func getRating(data []string, i int, comp func(int) bool) []string {

	zero, one := make([]string, 0), make([]string, 0)

	count := 0
	for _, d := range data {
		switch d[i] {
		case '1':
			count++
			one = append(one, d)
		case '0':
			count--
			zero = append(zero, d)
		}
	}

	if comp(count) {
		return one
	} else {
		return zero
	}
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
