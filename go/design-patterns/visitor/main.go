package main

import (
	"fmt"
	"strconv"
)

// Element 接口
type Element interface {
	Accept(v CarVisitor)
}

// Engine 为 ConcreteElement
type Engine struct{}

// Accept 操作接收一个 visitor 作为入参
func (e Engine) Accept(v CarVisitor) {
	v.visitEngine(e)
}

// Wheel 为 ConcreteElement
type Wheel struct {
	Number int
}

// Accept 操作
func (w Wheel) Accept(v CarVisitor) {
	v.visitWheel(w)
}

// Car 为 ConcreteElement
type Car struct {
	_items []Element
}

// Accept 操作
func (c Car) Accept(v CarVisitor) {
	for _, e := range c._items {
		e.Accept(v)
	}
	v.visitCar(c)
}

// CarVisitor 为 Visitor
type CarVisitor interface {
	visitEngine(engine Engine)
	visitWheel(wheel Wheel)
	visitCar(car Car)
}

// TestCarVisitor 为 ConcreteVisitor
type TestCarVisitor struct{}

func (v TestCarVisitor) visitEngine(engine Engine) {
	fmt.Println("test engine")
}

func (v TestCarVisitor) visitWheel(wheel Wheel) {
	fmt.Println("test wheel #" + strconv.Itoa(wheel.Number))
}

func (v TestCarVisitor) visitCar(car Car) {
	fmt.Println("test car")
}

// RepairCarVisitor is ConcreteVisitor
type RepairCarVisitor struct{}

func (v RepairCarVisitor) visitEngine(engine Engine) {
	fmt.Println("repair engine")
}

func (v RepairCarVisitor) visitWheel(wheel Wheel) {
	fmt.Println("repair wheel #" +
		strconv.Itoa(wheel.Number))
}

func (v RepairCarVisitor) visitCar(car Car) {
	fmt.Println("repair car")
}

func main() {
	car := Car{[]Element{
		Engine{},
		Wheel{1},
		Wheel{2},
		Wheel{3},
		Wheel{4},
	}}
	v1 := TestCarVisitor{}
	v2 := RepairCarVisitor{}

	car.Accept(v1)
	car.Accept(v2)
}
