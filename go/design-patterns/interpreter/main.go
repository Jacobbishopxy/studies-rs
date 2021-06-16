package main

import "fmt"

// Expression 作为 AbstractExpression
type Expression interface {
	Interpret(i int) bool
}

// DivExpression 作为 TerminalExpression
type DivExpression struct {
	divider int
}

// Interpret 函数计算 expression
func (e DivExpression) Interpret(i int) bool {
	return i%e.divider == 0
}

// OrExpression 作为 NonterminalExpression
type OrExpression struct {
	exp1 Expression
	exp2 Expression
}

// Interpret 函数计算 expression
func (e OrExpression) Interpret(i int) bool {
	return e.exp1.Interpret(i) || e.exp2.Interpret(i)
}

// AndExpression 作为 NonterminalExpression
type AndExpression struct {
	exp1 Expression
	exp2 Expression
}

// Interpret 函数计算 expression
func (e AndExpression) Interpret(i int) bool {
	return e.exp1.Interpret(i) && e.exp2.Interpret(i)
}

func main() {
	// 客户端
	divExp5 := DivExpression{5}
	divExp7 := DivExpression{7}
	orExp := OrExpression{divExp5, divExp7}
	andExp := AndExpression{divExp5, divExp7}

	//
	res1 := orExp.Interpret(21)

	//
	res2 := andExp.Interpret(21)

	//
	res3 := andExp.Interpret(35)

	fmt.Println(res1)
	fmt.Println(res2)
	fmt.Println(res3)
}
