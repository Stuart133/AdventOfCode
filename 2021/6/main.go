package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
	"strings"
)

var laternFishTimer = 6
var newLaternFishTimer = 8
var days = 256

func main() {
	c, err := ioutil.ReadFile("./data.txt")
	if err != nil {
		fmt.Printf("Error opening file: %v", err)
		os.Exit(1)
	}

	data := strings.Split(string(c), ",")
	fish := loadFish(data)

	for i := 0; i < days; i++ {
		fish = tick(fish)
	}

	fmt.Println(totalFish(fish))
}

func tick(fish map[int]int) map[int]int {
	newFish := map[int]int{}

	for k, v := range fish {
		if k == 0 {
			newFish[laternFishTimer] += v
			newFish[newLaternFishTimer] += v
		} else {
			newFish[k-1] += v
		}
	}

	return newFish
}

func loadFish(f []string) map[int]int {
	fish := map[int]int{}

	for i := range f {
		fish[getInt(f[i])]++
	}

	return fish
}

func totalFish(fish map[int]int) int {
	total := 0
	for _, v := range fish {
		total += v
	}

	return total
}

func getInt(s string) int {
	i, _ := strconv.Atoi(s)

	return i
}
