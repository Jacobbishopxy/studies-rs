package main

import "fmt"

// 原型 Prototype
type Shape interface {
	Clone() Shape
}

// 具体原型
type Square struct {
	LineCount int
}

// Square 实现 Shape
func (s Square) Clone() Shape {
	return Square{s.LineCount}
}

// 包含一个 Shape
type ShapeMaker struct {
	Shape Shape
}

// ShapeMaker 的方法
func (sm ShapeMaker) MakeShape() Shape {
	return sm.Shape.Clone()
}

func main() {
	square := Square{4}
	maker := ShapeMaker{square}

	square1 := maker.MakeShape()
	square2 := maker.MakeShape()

	fmt.Println(square1)
	fmt.Println(square2)
}
