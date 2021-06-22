package main

import "fmt"

// 抽象 Builder
type TextWorker interface {
	AddText(text string)
	AddNewLine(text string)
}

// 具体 builder 1
type TextBuilder struct {
	Text string
}

// TextBuilder 实现 TextWorker 的 AddText 方法
func (tb *TextBuilder) AddText(text string) {
	tb.Text += text
}

// TextBuilder 实现 TextWorker 的 AddNewLine 方法
func (tb *TextBuilder) AddNewLine(text string) {
	tb.Text += ("\n" + text)
}

// 具体的 builder 2
type HTMLBuilder struct {
	HTML string
}

// HTMLBuilder 实现 TextWorker 的 AddText 方法
func (tb *HTMLBuilder) AddText(text string) {
	tb.HTML += ("<span>" + text + "</span>")
}

// HTMLBuilder 实现 TextWorker 的 AddNewLine 方法
func (tb *HTMLBuilder) AddNewLine(text string) {
	tb.HTML += "<br/>\n"
	tb.AddText(text)
}

// 作为 Director
type TextMaker struct{}

// 具体过程
func (tm TextMaker) MakeText(textBuilder TextWorker) {
	textBuilder.AddText("line 1")
	textBuilder.AddNewLine("line 2")
}

func main() {
	textMaker := TextMaker{}

	textBuilder := TextBuilder{}
	textMaker.MakeText(&textBuilder)
	text := textBuilder.Text
	//text: line 1
	//      line 2

	htmlBuilder := HTMLBuilder{}
	textMaker.MakeText(&htmlBuilder)
	html := htmlBuilder.HTML
	//html: <span>line 1</span><br/>
	//      <span>line 2</span>

	fmt.Println(text)
	fmt.Println(html)
}
