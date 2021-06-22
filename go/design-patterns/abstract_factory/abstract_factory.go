package main

import "fmt"

// 抽象的 A
type ProductA interface {
	TestA()
}

// 抽象的 B
type ProductB interface {
	TestB()
}

// 抽象工厂
type Factory interface {
	CreateA() ProductA
	CreateB() ProductB
}

// 具体的 A1
type ProductA1 struct{}

// ProductA1 实现 ProductA 接口方法
func (p ProductA1) TestA() {
	fmt.Println("test A1")
}

// 具体的 B1
type ProductB1 struct{}

// ProductB1 实现 ProductB 接口方法
func (p ProductB1) TestB() {
	fmt.Println("test B1")
}

// 具体的 A2
type ProductA2 struct{}

// ProductA2 实现 ProductA 接口方法
func (p ProductA2) TestA() {
	fmt.Println("test A2")
}

// 具体的 B2
type ProductB2 struct{}

// ProductB2 实现 ProductB 接口方法
func (p ProductB2) TestB() {
	fmt.Println("test B2")
}

// 具体的 factory1
type Factory1 struct{}

// CreateA 实现 Factory 接口方法
func (f Factory1) CreateA() ProductA {
	return ProductA1{}
}

// CreateB 实现 Factory 接口方法
func (f Factory1) CreateB() ProductB {
	return ProductB1{}
}

// 具体的 factory2
type Factory2 struct{}

// CreateA 实现 Factory 接口方法
func (f Factory2) CreateA() ProductA {
	return ProductA1{}
}

// CreateB 实现 Factory 接口方法
func (f Factory2) CreateB() ProductB {
	return ProductB1{}
}

func TestFactory(factory Factory) {
	productA := factory.CreateA()
	productB := factory.CreateB()
	productA.TestA()
	productB.TestB()
}

// 抽象工厂，根据工厂具体类型来执行具体的函数
func main() {
	TestFactory(Factory1{})
	//printed: test A1
	//         test B1
	TestFactory(Factory2{})
	//printed: test A2
	//         test B2
}
