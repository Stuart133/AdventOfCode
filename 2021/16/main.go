package main

import (
	"fmt"
	"io/ioutil"
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
	value   int64
}

func (l literal) getVersionSum() int {
	return l.version
}

func (l literal) getValue() int64 {
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

func (o operator) getValue() int64 {
	fmt.Printf("%d - %v %d\n", o.typ, o.packets, len(o.packets))
	var calFunc func(a, b int64) int64
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
		if len(o.packets) != 2 {
			panic("OH NO")
		}

	case 6:
		calFunc = less
		if len(o.packets) != 2 {
			panic("OH NO")
		}

	case 7:
		calFunc = equal
		if len(o.packets) != 2 {
			panic("OH NO")
		}
	default:
		panic("OH NO")
	}

	total := o.packets[0].getValue()
	for i := 1; i < len(o.packets); i++ {
		// fmt.Printf("%d %d %d\n", o.typ, total, o.packets[i].getValue())
		total = calFunc(total, o.packets[i].getValue())
	}

	fmt.Printf("Total = %d\n", total)
	return total
}

func sum(a, b int64) int64 {
	return a + b
}

func product(a, b int64) int64 {
	return a * b
}

func min(a, b int64) int64 {
	if a < b {
		return a
	}

	return b
}

func max(a, b int64) int64 {
	if a > b {
		return a
	}

	return b
}

func greater(a, b int64) int64 {
	if a > b {
		return 1
	}

	return 0
}

func less(a, b int64) int64 {
	if a < b {
		return 1
	}

	return 0
}

func equal(a, b int64) int64 {
	if a == b {
		return 1
	}

	return 0
}

type packet interface {
	getVersionSum() int
	getValue() int64
}

func main() {
	c, err := ioutil.ReadFile("./data.txt")
	if err != nil {
		fmt.Printf("Error opening file: %v\n", err)
		os.Exit(1)
	}

	fmt.Println(getInt("10010011110011011011010011011010"))

	message := string(c)
	bits := bitString(message)
	p := parser{
		source:  bits,
		current: 0,
	}

	packets := p.parse()
	fmt.Println(packets[0].getValue())
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
	version := int(p.consumeValue(3))
	typ := int(p.consumeValue(3))

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

	digits := make([]int64, 0)
	valueString := ""
	for i := 0; true; i++ {
		isLastDigit := false
		if p.advance() == '0' {
			isLastDigit = true
		}
		digString := ""
		for i := 0; i < 4; i++ {
			bit := string(p.advance())
			digString += bit
			valueString += bit
		}
		digits = append(digits, getInt(digString))
		bits += 5

		if isLastDigit {
			fmt.Printf("%v %s %d\n", digits, valueString, getInt(valueString))
			l.value = getInt(valueString)
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

			if subBits == int(length) {
				break
			}
		}
	} else {
		count := p.consumeValue(11)
		bits += 11
		for i := 0; i < int(count); i++ {
			pack, len := p.parsePacket()
			bits += len
			o.packets = append(o.packets, pack)
		}
	}

	return o, bits
}

func (p *parser) consumeValue(length int) int64 {
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
	return p.current >= len(p.source)-1
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

func getInt(s string) int64 {
	i, _ := strconv.ParseInt(s, 2, 64)

	return i
}
