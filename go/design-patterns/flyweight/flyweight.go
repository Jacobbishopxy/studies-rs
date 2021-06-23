package main

import "fmt"

// 作为 Flyweight
type Span interface {
	PrintSpan(style string)
}

// 作为具体 flyweight
type Char struct {
	C rune
}

// 操作
func (c Char) PrintSpan(style string) {
	fmt.Println("<span style=\"" + style + "\">" + string(c.C) + "</span>")
}

// flyweight 工厂
type CharFactory struct {
	chars map[rune]Char
}

// 获取 flyweight
func (cf *CharFactory) GetChar(c rune) Span {
	if value, exists := cf.chars[c]; exists {
		return value
	}
	char := Char{c}
	cf.chars[c] = char
	return char
}

func main() {
	factory := CharFactory{map[rune]Char{}}
	charA := factory.GetChar('A')
	charA.PrintSpan("font-size: 12")

	charB := factory.GetChar('B')
	charB.PrintSpan("font-size: 12")

	charA1 := factory.GetChar('A')
	charA1.PrintSpan("font-size: 12")

	equal := charA == charA1

	fmt.Println(equal)
}
