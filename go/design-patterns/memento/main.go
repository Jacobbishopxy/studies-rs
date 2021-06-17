package main

import "fmt"

// Point 即 State
type Point struct {
	X, Y int
}

// Memento 包含一个 State 字段
type Memento struct {
	_state Point
}

func (m Memento) getState() Point {
	return m._state
}

// Shape 即 Originator
type Shape struct {
	Position Point
}

// Move Shape
func (s *Shape) Move(left, top int) {
	s.Position.X += left
	s.Position.Y += top
}

func (s *Shape) getMemento() Memento {
	state := Point{s.Position.X, s.Position.Y}
	return Memento{state}
}

func (s *Shape) setMemento(memento Memento) {
	s.Position = memento.getState()
}

// ShapeHelper 即 Caretaker
type ShapeHelper struct {
	_shape *Shape
	_stack []Memento
}

// ShapeHelper 构造器
func NewShapeHelper(shape *Shape) ShapeHelper {
	return ShapeHelper{shape, []Memento{}}
}

// 移动 Shape 同时存储优先状态
func (sh *ShapeHelper) Move(left, top int) {
	sh._stack = append(sh._stack, sh._shape.getMemento())
	sh._shape.Move(left, top)
}

// 回退 shape 至上一位置
func (sh *ShapeHelper) Undo() {
	l := len(sh._stack)
	if l > 0 {
		memento := sh._stack[l-1]
		sh._stack = sh._stack[:l-1]
		sh._shape.setMemento(memento)
	}
}

func main() {
	// 客户端
	shape := &Shape{}
	helper := NewShapeHelper(shape)

	helper.Move(2, 3)
	fmt.Println(shape.Position)
	helper.Move(-5, 4)
	fmt.Println(shape.Position)

	helper.Undo()
	fmt.Println(shape.Position)
	helper.Undo()
	fmt.Println(shape.Position)
}
