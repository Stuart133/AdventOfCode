package main

import (
	"fmt"
	"io/ioutil"
	"math"
	"os"
	"strconv"
	"strings"
)

func main() {
	c, err := ioutil.ReadFile("./data-smol.txt")
	if err != nil {
		fmt.Printf("Error opening file: %v", err)
		os.Exit(1)
	}

	rawData := strings.Split(string(c), ",")
	data := make([]int, len(rawData))
	total := 0
	for i := range rawData {
		data[i], _ = strconv.Atoi(rawData[i])
		total += data[i]
	}

	fmt.Println(total / len(data))
	avg := int(math.Round(float64(total) / float64(len(data))))
	fmt.Println(avg)
	fmt.Println(newFuelCost(data, total/len(data)))
}

func newFuelCost(c []int, p int) int {
	total := 0
	for i := range c {
		dist := abs(c[i] - p)
		total += (dist * (dist + 1)) / 2
	}

	return total
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
