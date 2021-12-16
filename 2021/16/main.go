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

type packet interface {
	getVersionSum() int
}

func main() {
	c, err := ioutil.ReadFile("./data-smol.txt")
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

	for i := 0; true; i++ {
		isLastDigit := false
		if p.advance() == '0' {
			isLastDigit = true
		}
		for i := 0; i < 4; i++ {
			p.advance()
		}
		bits += 5

		if isLastDigit {
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

func pow(a, b int) int64 {
	return int64(math.Pow(float64(a), float64(b)))
}

func getInt(s string) int {
	i, _ := strconv.ParseInt(s, 2, 32)

	return int(i)
}
