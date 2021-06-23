package main

import (
	"fmt"
	"strings"
)

// 作为 adaptee
type StringList struct {
	rows []string
}

func (sl StringList) getString() string {
	return strings.Join(sl.rows, "\n")
}

func (sl *StringList) add(value string) {
	sl.rows = append(sl.rows, value)
}

// 作为 adapter
type TextAdapter struct {
	RowList StringList
}

func (ta TextAdapter) getText() string {
	return ta.RowList.getString()
}

func getTextAdapter() TextAdapter {
	rowList := StringList{}
	rowList.add("line 1")
	rowList.add("line 2")
	adapter := TextAdapter{rowList}
	return adapter
}

func main() {
	adapter := getTextAdapter()
	text := adapter.getText()

	fmt.Println(text)
}
