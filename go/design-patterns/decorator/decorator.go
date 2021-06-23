package main

import "fmt"

// 作为 Component
type Shape interface {
	ShowInfo()
}

// 作为具体 Component
type Square struct{}

// 操作
func (s Square) ShowInfo() {
	fmt.Print("square")
}

// 作为 Decorator
type ShapeDecorator struct {
	Shape Shape
}

// 操作
func (sd ShapeDecorator) ShowInfo() {
	sd.Shape.ShowInfo()
}

// 作为具体 Decorator
type ColorShape struct {
	ShapeDecorator
	color string
}

// 操作
func (cs ColorShape) ShowInfo() {
	fmt.Print(cs.color + " ")
	cs.Shape.ShowInfo()
}

// 作为具体 Decorator
type ShadowShape struct {
	ShapeDecorator
}

// 操作
func (ss ShadowShape) ShowInfo() {
	ss.Shape.ShowInfo()
	fmt.Print(" with shadow")
}

func main() {
	square := Square{}
	square.ShowInfo()
	fmt.Println()

	colorShape := ColorShape{
		ShapeDecorator{square}, "red",
	}
	colorShape.ShowInfo()
	fmt.Println()

	shadowShape := ShadowShape{
		ShapeDecorator{colorShape},
	}
	shadowShape.ShowInfo()
}
