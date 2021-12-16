package main

import (
	"fmt"
	"io/ioutil"
	"os"
)

func main() {
	c, err := ioutil.ReadFile("./data-smol.txt")
	if err != nil {
		fmt.Printf("Error opening file: %v\n", err)
		os.Exit(1)
	}

	message := string(c)
	fmt.Println(bitString(message))

}

func bitString(s string) string {
	bits := ""
	for i := range s {
		bits += convertChar(s[i])
	}

	return bits
}

func convertChar(c byte) string {
	if c > 47 && c < 58 {
		return fmt.Sprintf("%04b", c-48)
	}

	return fmt.Sprintf("%04b", c-55)
}
