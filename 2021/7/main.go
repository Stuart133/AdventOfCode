package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	c, err := ioutil.ReadFile("./data.txt")
	if err != nil {
		fmt.Printf("Error opening file: %v", err)
		os.Exit(1)
	}

	rawData := strings.Split(string(c), ",")
	data := make([]int, len(rawData))
	for i := range rawData {
		data[i], _ = strconv.Atoi(rawData[i])
	}
	sort.Ints(data)

	if (len(data)/2)%2 != 0 {
		fmt.Printf("Median: %d\n", data[len(data)/2])
	} else {
		fmt.Printf("Medians: %d, %d\n", data[len(data)/2], data[len(data)/2+1])
	}

	cost := fuelCost(data, data[len(data)/2+1])
	fmt.Println(cost)
}

func fuelCost(c []int, p int) int {
	total := 0
	for i := range c {
		total += abs(c[i] - p)
	}

	return total
}

func abs(x int) int {
	if x < 0 {
		return -x
	}

	return x
}
