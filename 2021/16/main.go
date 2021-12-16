package main

import (
	"fmt"
	"io/ioutil"
	"math"
	"os"
	"strconv"
)

type parser struct {
	source  string
	current int
}

type literal struct {
	version int
	typ     int
	value   int
}

func (l literal) getVersionSum() int {
	return l.version
}

func (l literal) getValue() int {
	return l.value
}

type operator struct {
	version int
	typ     int
	packets []packet
}

func (o operator) getVersionSum() int {
	total := 0
	for i := range o.packets {
		total += o.packets[i].getVersionSum()
	}

	return o.version + total
}

func (o operator) getValue() int {
	var calFunc func(a, b int) int
	switch o.typ {
	case 0:
		calFunc = sum
	case 1:
		calFunc = product
	case 2:
		calFunc = min
	case 3:
		calFunc = max
	case 5:
		calFunc = greater
	case 6:
		calFunc = less
	case 7:
		calFunc = equal
	}

	total := o.packets[0].getValue()
	for i := 1; i < len(o.packets); i++ {
		// fmt.Printf("%d %d %d\n", o.typ, total, o.packets[i].getValue())
		total = calFunc(total, o.packets[i].getValue())
	}

	return total
}

func sum(a, b int) int {
	return a + b
}

func product(a, b int) int {
	return a * b
}

func min(a, b int) int {
	if a < b {
		return a
	}

	return b
}

func max(a, b int) int {
	if a > b {
		return a
	}

	return b
}

func greater(a, b int) int {
	if a > b {
		return 1
	}

	return 0
}

func less(a, b int) int {
	if a < b {
		return 1
	}

	return 0
}

func equal(a, b int) int {
	if a == b {
		return 1
	}

	return 0
}

type packet interface {
	getVersionSum() int
	getValue() int
}

func main() {
	c, err := ioutil.ReadFile("./data.txt")
	if err != nil {
		fmt.Printf("Error opening file: %v\n", err)
		os.Exit(1)
	}

	message := string(c)
	bits := bitString(message)
	p := parser{
		source:  bits,
		current: 0,
	}

	packets := p.parse()
	fmt.Println(len(packets))
	fmt.Println(packets[0].getValue())
	fmt.Println(packets[0].getVersionSum())
}

func (p *parser) parse() []packet {
	packets := make([]packet, 0)

	for !p.isAtEnd() {
		pack, _ := p.parsePacket()
		packets = append(packets, pack)
	}

	return packets
}

func (p *parser) parsePacket() (packet, int) {
	version := p.consumeValue(3)
	typ := p.consumeValue(3)

	if typ == 4 {
		return p.parseLiteral(version)
	} else {
		return p.parseOperator(version, typ)
	}
}

func (p *parser) parseLiteral(version int) (literal, int) {
	l := literal{
		version: version,
		typ:     4}

	bits := 6

	digits := make([]int, 0)
	for i := 0; true; i++ {
		isLastDigit := false
		if p.advance() == '0' {
			isLastDigit = true
		}
		digits = append(digits, p.consumeValue(4))
		bits += 5

		if isLastDigit {
			l.value = getValue(digits)
			break
		}
	}

	return l, bits
}

func (p *parser) parseOperator(version int, typ int) (operator, int) {
	o := operator{
		version: version,
		typ:     typ,
		packets: make([]packet, 0),
	}
	bits := 7

	if p.advance() == '0' {
		length := p.consumeValue(15)
		bits += 15
		subBits := 0
		for {
			pack, len := p.parsePacket()
			o.packets = append(o.packets, pack)
			subBits += len
			bits += len

			if subBits == length {
				break
			}
		}
	} else {
		count := p.consumeValue(11)
		bits += 11
		for i := 0; i < count; i++ {
			pack, len := p.parsePacket()
			bits += len
			o.packets = append(o.packets, pack)
		}
	}

	return o, bits
}

func (p *parser) consumeValue(length int) int {
	valString := ""
	for i := 0; i < length; i++ {
		valString += string(p.advance())
	}

	return getInt(valString)
}

func (p *parser) advance() byte {
	b := p.source[p.current]
	p.current++

	return b
}

func (p *parser) isAtEnd() bool {
	return p.current >= len(p.source)-10
}

func getValue(digits []int) int {
	val := 0
	for i := range digits {
		val += digits[i] * pow(10, len(digits)-i-1)
	}

	return val
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

func pow(a, b int) int {
	return int(math.Pow(float64(a), float64(b)))
}

func getInt(s string) int {
	i, _ := strconv.ParseInt(s, 2, 32)

	return int(i)
}
