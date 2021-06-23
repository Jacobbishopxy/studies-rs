package main

import (
	"fmt"
	"strings"
)

// 作为 Abstraction
type AText interface {
	getText() string
	addLine(value string)
}

// 作为抽象的 Implementator
type ITextImp interface {
	getString() string
	appendLine(value string)
}

// 作为 Implementator
type TextImp struct {
	rows []string
}

func (ti TextImp) getString() string {
	return strings.Join(ti.rows, "\n")
}

// 作为 RefinedAbstraction
type TextMaker struct {
	textImp ITextImp
}

func (tm TextMaker) getText() string {
	return tm.textImp.getString()
}

func (tm TextMaker) addLine(value string) {
	tm.textImp.appendLine(value)
}

// 作为具体 Implementator1
type TextBuilder struct {
	TextImp
}

func (tb *TextBuilder) appendLine(value string) {
	tb.rows = append(tb.rows, value)
}

// 作为具体 Implementator2
type HTMLBuilder struct {
	TextImp
}

func (hb *HTMLBuilder) appendLine(value string) {
	hb.rows = append(hb.rows, "<span>"+value+"</span><bt/>")
}

func fillTextBuilder(textImp ITextImp) AText {
	textMaker := TextMaker{textImp}
	textMaker.addLine("line 1")
	textMaker.addLine("line 2")
	return textMaker
}

func main() {
	textMaker := fillTextBuilder(&TextBuilder{})
	text := textMaker.getText()

	htmlMaker := fillTextBuilder(&HTMLBuilder{})
	html := htmlMaker.getText()

	fmt.Println(text)
	fmt.Println(html)
}
