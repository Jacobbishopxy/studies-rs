package main

import "fmt"

// Strategy 带有整数操作
type Strategy interface {
	DoOperation(a, b int) int
}

// AddStrategy 为 ConcreteStrategy
type AddStrategy struct{}

// DoOperation 为整数加法函数
func (s AddStrategy) DoOperation(a, b int) int {
	return a + b
}

// SubstractStrategy 为 ConcreteStrategy
type SubstractStrategy struct{}

// DoOperation 为整数减法函数
func (s SubstractStrategy) DoOperation(a, b int) int {
	return a - b
}

// Calc 为 Context
type Calc struct {
	_strategy Strategy
}

// Execute 当前的 strategy
func (c Calc) Execute(a, b int) int {
	if c._strategy == nil {
		return 0
	}

	return c._strategy.DoOperation(a, b)
}

// SetStrategy 改变当前 strategy
func (c *Calc) SetStrategy(strategy Strategy) {
	c._strategy = strategy
}

func main() {
	calc := Calc{}

	result1 := calc.Execute(5, 3)
	// result1 为 0
	fmt.Println(result1)

	calc.SetStrategy(AddStrategy{})
	result2 := calc.Execute(5, 3)
	// result2 为 8
	fmt.Println(result2)

	calc.SetStrategy(SubstractStrategy{})
	result3 := calc.Execute(5, 3)
	// result3 为 2
	fmt.Println(result3)
}
